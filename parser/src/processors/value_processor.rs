use crate::parser;
use crate::processors::type_processors;
use crate::syntax::{types, variable};
use ellie_core::error;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, PartialEq)]
pub struct CollectorResponse {
    pub itered_data: variable::VariableCollector,
    pub errors: Vec<error::Error>,
}

pub fn collect_value(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    letter_char: &str,
    next_char: String,
    last_char: String,
) -> CollectorResponse {
    let mut errors: Vec<error::Error> = Vec::new();
    match &mut itered_data.data.value {
        types::Types::Integer(_) => type_processors::integer::collect_integer(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::Float(_) => type_processors::float::collect_float(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::Bool(_) => type_processors::bool::collect_bool(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::String(_) => type_processors::string::collect_string(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::Char(_) => type_processors::char::collect_char(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::Collective => (),
        types::Types::Refference(_) => type_processors::refference::collect_refference(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::Operator(_) => type_processors::operator::collect_operator(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::Array(_) => type_processors::array::collect_array(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::Cloak(_) => type_processors::cloak::collect_cloak(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::ArrowFunction(_) => type_processors::arrow_function::collect_arrow(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::FunctionCall(_) => type_processors::function_call::collect_function_caller(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::ClassCall(_) => type_processors::class_call::collect_class_call(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::Void => (),
        types::Types::VariableType(_) => type_processors::variable::collect_variable(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
        types::Types::Null => type_processors::null::collect_null(
            parser,
            itered_data,
            &mut errors,
            letter_char,
            next_char,
            last_char,
        ),
    }
    CollectorResponse {
        itered_data: itered_data.clone(),
        errors,
    }
}
