use colorterm::*;

pub fn pretty_analysis_result(
    analysis: &model::SituationAnalysisResult,
    color: &model::Color,
) -> String {
    let player_has_moves_left = analysis
        .possible_after_move()
        .as_ref()
        .map(|s| s.items().len() > 0);

    let available = match player_has_moves_left {
        Some(false) => {
            return match (color, analysis.white_in_check(), analysis.black_in_check()) {
                (model::Color::White, true, false) => {
                    format!("{COLOR_BRIGHT_WHITE}white is {STYLE_BOLD}checkmated{STYLE_RESET}{COLOR_RESET}")
                }
                (model::Color::Black, false, true) => {
                    format!("{COLOR_BLUE}black is {STYLE_BOLD}checkmated{STYLE_RESET}{COLOR_RESET}")
                }
                (_, false, false) => {
                    format!("{COLOR_GREEN}{STYLE_BOLD}stalemate{STYLE_RESET}{COLOR_RESET}")
                }
                _ => format!("{COLOR_RED}this board is in weird shape!{COLOR_RESET}"),
            }
        }
        Some(true) => true,
        None => false,
    };

    let no_checkmate_information = if !available {
        format!(", {COLOR_BRIGHT_BLACK}no checkmate information available{COLOR_RESET}")
    } else {
        "".to_string()
    };
    let no_stalemate_information = if !available {
        format!(", {COLOR_BRIGHT_BLACK}no stalemate information available{COLOR_RESET}")
    } else {
        "".to_string()
    };
    let in_check = match (analysis.white_in_check(), analysis.black_in_check()) {
        (true, true) => {
            format!("{COLOR_MAGENTA}white and black in check board is in weird shape?{COLOR_RESET}")
        }

        (true, false) => {
            format!("{COLOR_BRIGHT_WHITE}{STYLE_BOLD}white in check{STYLE_RESET}{COLOR_RESET}{no_checkmate_information}")
        }
        (false, true) => {
            format!("{COLOR_BLUE}{STYLE_BOLD}black in check{COLOR_RESET}{STYLE_RESET}{no_checkmate_information}")
        }
        (false, false) => {
            format!("{COLOR_BRIGHT_BLACK}noone in check{COLOR_RESET}{no_stalemate_information}")
        }
    };
    in_check
}
