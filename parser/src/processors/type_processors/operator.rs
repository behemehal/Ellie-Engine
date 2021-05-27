use crate::processors::value_processor;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_operator(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
<<<<<<< HEAD
    options: defs::ParserOptions,
=======
    options: defs::ParserOptions
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
) {
    if let types::Types::Operator(ref mut data) = itered_data.data.value {
        if !data.operator_collected {
            //Operator
            let is_opearator = types::operator_type::Operators::resolve_operator(
                data.operator.clone(),
                &(data.operator_collect.clone() + letter_char),
            );
            if is_opearator.is_err() {
                if letter_char == " " {
                    data.operator_collected = true;
                } else {
                    errors.push(error::Error {
                        debug_message: "./parser/src/processors/type_processors/operator.rs:29" .to_string(),
                        title: error::errorList::error_s13.title.clone(),
                        code: error::errorList::error_s13.code,
                        message: error::errorList::error_s13.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s13.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
            } else if let Ok(parsed_operator) = is_opearator {
                data.operator_collect += letter_char;
                if let types::operator_type::Operators::ComparisonType(_) = data.operator {
                    data.operator = parsed_operator;
                } else {
                    data.operator = parsed_operator;
                }
            }
        } else {
            //Second
            let mut will_be_itered = data.itered_cache.clone();
            data.second_is_not_null = true;
            let itered_child = value_processor::collect_value(
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
                defs::CursorPosition(0, 0),
<<<<<<< HEAD
                options,
=======
                options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
            );
            if itered_child.errors.is_empty() {
                for returned_error in itered_child.errors {
                    let mut edited = returned_error;
                    edited.pos.range_start.0 += pos.0;
                    edited.pos.range_start.1 += pos.1;
                    edited.pos.range_end.0 += pos.0;
                    edited.pos.range_end.1 += pos.1;
                    errors.push(edited);
                }
            }
            if let types::Types::Operator(child_operator) =
                itered_child.itered_data.data.value.clone()
            {
                if child_operator.operator == data.operator {
                    itered_data.data.value =
                        types::Types::Operator(types::operator_type::OperatorType {
                            cloaked: child_operator.cloaked,
                            first: Box::new(types::Types::Operator(
                                types::operator_type::OperatorType {
                                    cloaked: data.cloaked,
                                    first: data.first.clone(),
                                    first_filled: true,
                                    second: child_operator.first,
                                    operator: data.operator.clone(),
                                    operator_collect: data.operator_collect.clone(),
                                    operator_collected: true,
                                    ..Default::default()
                                },
                            )),
                            first_filled: true,
                            operator: child_operator.operator,
                            operator_collect: child_operator.operator_collect,
                            ..Default::default()
                        })
                } else {
                    match data.operator.clone() {
                        types::operator_type::Operators::ComparisonType(_) => {
                            if child_operator.second == Box::new(types::Types::Null) {}
                            itered_data.data.value =
                                types::Types::Operator(types::operator_type::OperatorType {
                                    cloaked: data.cloaked,
                                    first: Box::new(types::Types::Operator(
                                        types::operator_type::OperatorType {
                                            first_filled: true,
                                            cloaked: data.cloaked,
                                            first: data.first.clone(),
                                            second: child_operator.first.clone(),
                                            operator: data.operator.clone(),
                                            operator_collect: data.operator_collect.clone(),
                                            operator_collected: true,
                                            ..Default::default()
                                        },
                                    )),
                                    first_filled: true,
                                    operator: child_operator.operator,
                                    operator_collect: child_operator.operator_collect,
                                    ..Default::default()
                                })
                        }
                        _ => {
                            data.second = Box::new(itered_child.itered_data.data.value.clone());
                            data.itered_cache = Box::new(itered_child.itered_data);
                        }
                    }
                }
            } else {
                data.itered_cache = Box::new(itered_child.itered_data.clone());
                data.second = Box::new(itered_child.itered_data.data.value);
            }
        }
    }
}

