use std::sync::Arc;

use abstraction::{FromCppTypes, ParsingRulesDecl};
use itertools::Itertools;

use crate::extract::ExtractedOutput;
use crate::flags::ManageCompilerFlags;
use crate::ExtraCompilerFlags;
use crate::Rule;

use super::extract::get_output_lines;
use super::run_gpp;
use super::Error;

/// Abstracts a C++ compiler with predefined additional library paths
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Compiler {
    compiler_flags: ExtraCompilerFlags,
}

impl From<ExtraCompilerFlags> for Compiler {
    fn from(value: ExtraCompilerFlags) -> Self {
        Self {
            compiler_flags: value,
        }
    }
}
impl ManageCompilerFlags for Compiler {
    fn into_flags(self) -> ExtraCompilerFlags {
        self.compiler_flags
    }
}

impl Compiler {
    pub fn new_with(compiler_flags: ExtraCompilerFlags) -> Self {
        Self { compiler_flags }
    }
    pub fn new() -> Self {
        Self {
            compiler_flags: Default::default(),
        }
    }

    /// Interpolates the provided expression into a temporary C++ source file and compile it with
    /// the stored include paths. Returns the expression and the output
    pub fn invoke(&self, type_expression: impl Into<Arc<str>>) -> Result<Invoke, Error> {
        let type_expression = type_expression.into();
        let (raw_text, output) = invoke_expr4output(&type_expression, &self.compiler_flags)?;
        Ok(Invoke {
            type_expression,
            raw_text,
            output,
        })
    }
    /// Same as [Self::invoke], but try to parse the output type with the provided grammar rule
    pub fn parsed_with_rule<T: FromCppTypes<Rule> + ParsingRulesDecl<Rule>>(
        &self,
        type_expression: impl Into<Arc<str>>,
        rule: super::Rule,
    ) -> Result<T, Error> {
        self.invoke(type_expression)?.parsed_with_rule(rule)
    }
    /// Same as [Self::parsed_with_rule], but uses the rule that is associated with the generic
    /// target type.
    ///
    /// The type parameter `T` will be specified by the return type, so the caller needs to provide
    /// the typing information for the compiler.
    ///
    /// `T` needs to implement [FromCppTypes][abstraction::FromCppTypes] and
    /// [ParsingRulesDecl][abstraction::ParsingRulesDecl] for
    /// the grammar rule type [Rule][model::Rule]
    pub fn parsed<T: FromCppTypes<Rule> + ParsingRulesDecl<Rule>>(
        &self,
        type_expression: impl Into<Arc<str>>,
    ) -> Result<T, Error> {
        self.parsed_with_rule(
            type_expression,
            <T as ParsingRulesDecl<Rule>>::_const_main_rule(),
        )
    }
}

/// Stores a type expression together with the associated output
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Invoke {
    type_expression: Arc<str>,
    raw_text: Arc<str>,
    output: ExtractedOutput,
}

impl Invoke {
    pub fn type_expression(&self) -> Arc<str> {
        self.type_expression.clone()
    }
    pub fn output(&self) -> ExtractedOutput {
        self.output.clone()
    }
    pub fn raw_text(&self) -> Arc<str> {
        self.raw_text.clone()
    }
    /// Same as [self.invoke], but try to parse the output type with the provided grammar rule
    pub fn parsed_with_rule<T: FromCppTypes<Rule> + ParsingRulesDecl<Rule>>(
        &self,
        rule: super::Rule,
    ) -> Result<T, Error> {
        use pest::Parser;
        let parsed = super::ChessParser::parse(rule, &self.output)
            .map_err(Error::Parsing)?
            .next()
            .ok_or(Error::ParsingAbstraction)?;

        T::try_from_cpp(parsed).ok_or(Error::ParsingAbstraction)
    }
    /// Same as [self.parsed_with_rule], but use the rule that is associated with the generic
    /// target type.
    ///
    /// The type parameter `T` will be specified by the return type, so the caller needs to provide
    /// the typing information for the compiler.
    ///
    /// `T` needs to implement [FromCppTypes][abstraction::FromCppTypes] and
    /// [ParsingRulesDecl][abstraction::ParsingRulesDecl] for
    /// the grammar rule type [Rule][model::Rule]
    pub fn parsed<T: FromCppTypes<Rule> + ParsingRulesDecl<Rule>>(&self) -> Result<T, Error> {
        self.parsed_with_rule(<T as ParsingRulesDecl<Rule>>::_const_main_rule())
    }
}

fn invoke4output(
    input_file_content: &str,
    compiler_flags: &ExtraCompilerFlags,
) -> Result<(Arc<str>, Vec<ExtractedOutput>), Error> {
    run_gpp(input_file_content, compiler_flags)
        .map(|text| (text.clone(), get_output_lines(&text).collect_vec()))
}

fn invoke_expr4output(
    type_expression: &str,
    compiler_flags: &ExtraCompilerFlags,
) -> Result<(Arc<str>, ExtractedOutput), Error> {
    let res = invoke4output(
        &format!(
            "#include \"chess/default.h\"

namespace __chess_output_helper {{
    struct Output {{ }};
}}

int main() {{
    __chess_output_helper::Output _ = identity<{}> {{}};
}}
",
            type_expression
        ),
        compiler_flags,
    )?;
    match res.1.iter().exactly_one() {
        Err(_) => Err(Error::WrongNumberOfOutputs(res.0, res.1.clone())),
        Ok(extracted) => Ok((res.0, extracted.clone())),
    }
}
