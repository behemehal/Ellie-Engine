use crate::parser::Collecting;
use crate::syntax::{function, r#type};
use serde::Serialize;

//use ellie_core::{defs};
//use alloc::string::String;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ArrowFunction {
    pub parameters: Vec<function::FunctionParameterCollector>,
    pub return_type: Box<r#type::Collecting>,
    pub inside_code: Vec<Collecting>,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ArrowFunctionCollector {
    pub complete: bool,
    pub param_bracket_opened: bool,
    pub parameter_wrote: bool,
    pub pointer_typed: bool,
    pub inside_code_string: String,
    pub return_typed: bool,
    pub brace_count: i64,
    pub data: ArrowFunction,
}
