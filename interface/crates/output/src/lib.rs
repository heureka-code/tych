use analysis::pretty_analysis_result;
use colorterm::*;

mod analysis;
mod any_move;
mod board;

pub trait PrettyPrint {
    fn pretty(&self) -> String;
}

fn print_initial_situation(steps: &model::MultiExecutionResult) -> String {
    let initial_situation = steps.initial_situation();
    format!(
        "Initial board (current is {}): \n{}",
        initial_situation.current_color(),
        initial_situation.board().pretty()
    )
}

pub fn print_multiple_steps(args: &cli::Cli, steps: &model::MultiExecutionResult) -> String {
    let mut output = vec![];

    if args.print_board.show_initial_board() {
        output.push(print_initial_situation(steps));
    }

    let items = steps.steps().items();
    for (index, step) in items.iter().enumerate() {
        let is_last = index + 1 == items.len();

        let validation = step
            .validation()
            .as_ref()
            .map(|_| format!("{COLOR_BRIGHT_GREEN}{}{COLOR_RESET}", "Accepted"))
            .map_err(|err| format!("{COLOR_BRIGHT_RED}Rejected: {:?}{COLOR_RESET}", err));

        let analysis = step.analysis();
        let in_check = pretty_analysis_result(analysis, step.after().current_color());

        let fmt_move = format!("{}: ", step.tried_move().pretty());

        output.push(match validation {
            Ok(msg) => format!(
                "{:<8}{msg}, {in_check}{}",
                fmt_move,
                if (is_last && args.print_board.show_last_board())
                    || (!is_last && args.print_board.show_intermediate_board())
                {
                    format!("\n{}", step.after().board().pretty())
                } else {
                    "".to_string()
                }
            ),
            Err(msg) => format!("{:<8}{msg}, {in_check}", fmt_move),
        });
    }

    if args.print_board.show_intermediate_board() {
        output.join(
            "\n------------------------------------------------------------------------------\n",
        )
    } else {
        output.join("\n")
    }
}
