use crate::error;
use crate::mapper;
use crate::syntax::types;
use crate::syntax::variable;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct CollectorResponse {
    pub itered_data: variable::VariableCollector,
    pub errors: Vec<error::Error>,
}

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: mapper::defs::CursorPosition,
) -> CollectorResponse {
    let mut errors: Vec<error::Error> = Vec::new();
    match &mut itered_data.value {
        types::Types::Number(data) => {
            let is_num = letter_char.parse::<usize>().is_ok();
            if is_num {
                if data.complete {
                    errors.push(error::Error {
                        debug_message: "Caria".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.value = (data.value.to_string() + &letter_char)
                        .parse::<usize>()
                        .unwrap();
                }
            } else {
                if letter_char == "." {
                    // String prototype
                    itered_data.value = types::Types::Refference(types::RefferenceType {
                        refference: Box::new(itered_data.value.clone()),
                        on_dot: true,
                        chain: Vec::new(),
                    });
                } else if utils::is_opearators(letter_char) {
                    //itered_data.value = types::Types::Operators(types::OperatorType {
                    //    first: Box::new(itered_data.value.clone()),
                    //    operator_collect: letter_char.to_string(),
                    //    collecting_operator: true,
                    //    ..Default::default()
                    //});
                } else if letter_char == " " {
                    data.complete = true;
                } else {
                    errors.push(error::Error {
                        debug_message: "Trtex".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
            }
            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Double(_) => CollectorResponse {
            itered_data: itered_data.clone(),
            errors,
        },
        types::Types::Bool(_) => CollectorResponse {
            itered_data: itered_data.clone(),
            errors,
        },
        types::Types::String(data) => {
            if letter_char == data.quote_type && last_char != "\\" {
                if data.complete {
                    errors.push(error::Error {
                        debug_message: "\\src\\processors\\value_processor.rs::86:0".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.complete = true;
                }
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            } else if letter_char == "." {
                // String prototype
                itered_data.value = types::Types::Refference(types::RefferenceType {
                    refference: Box::new(itered_data.value.clone()),
                    on_dot: true,
                    chain: Vec::new(),
                });
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            } else if utils::is_opearators(letter_char) {
                //itered_data.value = types::Types::Operators(types::OperatorType {
                //    first: Box::new(itered_data.value.clone()),
                //    operator_collect: letter_char.to_string(),
                //    collecting_operator: true,
                //    ..Default::default()
                //});
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            } else if letter_char != "\\" {
                data.value = data.value.clone() + &letter_char;
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            } else {
                CollectorResponse {
                    itered_data: itered_data.clone(),
                    errors,
                }
            }
        }
        types::Types::Collective(data) => {
            if data.collective.len() == 0 {
                //Collective just started
                if letter_char == " " {
                } else {
                    let current_reliability = crate::utils::reliable_name_range(
                        crate::utils::ReliableNameRanges::VariableName,
                        letter_char.to_string(),
                    );

                    if current_reliability.reliable {
                        data.collective.push(types::CollectiveEntry {
                            key: letter_char.to_string(),
                            dynamic: false,
                            key_named: false,
                            r#type: String::from(""),
                            typed: false,
                            value_complete: false,
                            raw_value: String::from(""),
                            value: Box::new(types::Types::Null),
                        })
                    } else {
                        errors.push(error::Error {
                            debug_message: "\\src\\mapper\\mod.rs:227:0".to_string(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                            ),
                            pos: mapper::defs::Cursor {
                                range_start: pos.clone(),
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                }
            } else {
                let last_entry = data.collective.len() - 1;

                if !data.collective[last_entry].key_named {
                    if letter_char == "=" {
                        //TODO: Do we have to check collective entry named? or unnamed entry cannot pass first if?
                        data.collective[last_entry].key_named = true;
                        data.collective[last_entry].dynamic = true;
                    } else if letter_char == ":" {
                        data.collective[last_entry].key_named = true;
                    } else {
                        let current_reliability = crate::utils::reliable_name_range(
                            crate::utils::ReliableNameRanges::VariableName,
                            letter_char.to_string(),
                        );

                        if current_reliability.reliable {
                            data.collective[last_entry].key += letter_char;
                        } else {
                            errors.push(error::Error {
                                debug_message: "\\src\\mapper\\mod.rs:227:0".to_string(),
                                title: error::errorList::error_s1.title.clone(),
                                code: error::errorList::error_s1.code,
                                message: error::errorList::error_s1.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s1.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_string(),
                                        value: letter_char.to_string(),
                                    }],
                                ),
                                pos: mapper::defs::Cursor {
                                    range_start: pos.clone(),
                                    range_end: pos.clone().skipChar(1),
                                },
                            });
                        }
                    }
                } else {
                    if data.collective[last_entry].dynamic {
                        println!(
                            "{:#?}",
                            collect(
                                &mut itered_data.clone(),
                                letter_char.clone(),
                                next_char.to_string().clone(),
                                last_char.to_string().clone(),
                                pos.clone(),
                            )
                        );
                    //println!("CollectiveData: {:#?}", collected);
                    } else {
                        //We're expecting data type
                        println!("Non-dynamic collection entry not supported yet?");
                    }
                }
            }

            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Refference(data) => {
            if letter_char == "." {
                if data.on_dot {
                    errors.push(error::Error {
                        debug_message: "\\src\\processors\\value_processor.rs::127:0".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.on_dot = true;
                }
            } else if letter_char != " " {
                if data.on_dot {
                    data.on_dot = false;
                    data.chain.push(letter_char.to_string());
                } else if last_char == " "
                    && data.chain.len() != 0
                    && data.chain[data.chain.len() - 1] != ""
                {
                    if utils::is_opearators(letter_char) {
                        //itered_data.value = types::Types::Operators(types::OperatorType {
                        //    first: Box::new(itered_data.value.clone()),
                        //    operator_collect: letter_char.to_string(),
                        //    collecting_operator: true,
                        //    ..Default::default()
                        //});
                    } else {
                        errors.push(error::Error {
                            debug_message: "\\src\\processors\\value_processor.rs::127:0"
                                .to_string(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                            ),
                            pos: mapper::defs::Cursor {
                                range_start: pos.clone(),
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else {
                    let chain_last_element = data.chain.len() - 1;
                    data.chain[chain_last_element] =
                        data.chain[chain_last_element].clone() + &letter_char;
                }
            }
            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Array(data) => {
            /*
                Don't look right to it, it's dangerously complicated
                Here is the story,

                I assume you as a person that doesn't have a programming experience. In a loop you can process a data
                and if a same data applied you can use the same function to process the data, This program uses millions of same pattern,
                I experienced this a million times, Created programs that runs through loops processing big data. But this time I got stuck at this
                function. It took almost 2 months, Thank god I got it.

                A Weird way to stop a letter,

                Sincerely

                Ahmetcan Aksu 🦀
            */

            let last_entry = data.clone().collective.len();
            let mut value: types::Types = types::Types::Null;

            let is_s_n = if last_entry != 0
                && data.collective[last_entry - 1]
                    .value
                    .is_string_non_complete()
            {
                false
            } else {
                true
            };

            if letter_char == "[" && !data.child_start && is_s_n {
                if !data.comma {
                    errors.push(error::Error {
                        debug_message: "Tette".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.child_start = true;
                    value = types::Types::Array(types::ArrayType::default());
                    data.collective[last_entry - 1] = types::ArrayEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Array(types::ArrayType::default())),
                    };
                }
            } else if letter_char == "," && !data.child_start && is_s_n {
                if data.complete {
                    errors.push(error::Error {
                        debug_message: "\\src\\processors\\value_processor.rs::320:0".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else if data.comma {
                    errors.push(error::Error {
                        debug_message: "\\src\\processors\\value_processor.rs::338:0".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    if last_entry != 0 {
                        data.collective[last_entry - 1].value.make_complete();
                        data.collective[last_entry - 1].value_complete = true;
                    }
                    data.comma = true;
                    data.layer_size += 1;
                    data.collective.push(types::ArrayEntry::default());
                }
            } else if letter_char == "]" && !data.child_start && is_s_n {
                if data.comma {
                    errors.push(error::Error {
                        debug_message: "Tretra".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else if data.complete {
                    errors.push(error::Error {
                        debug_message: "Nonntkr".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    if last_entry != 0 {
                        if data.collective[last_entry - 1].value == Box::new(types::Types::Null) {
                            data.collective.pop();
                        } else {
                            data.collective[last_entry - 1].value_complete = true;
                            data.collective[last_entry - 1].value.make_complete();
                        }
                    }
                    data.layer_size += 1;
                    data.complete = true;
                    itered_data.value_complete = true;
                }
            } else if data.complete && letter_char == "." && is_s_n {
                itered_data.value = types::Types::Refference(types::RefferenceType {
                    refference: Box::new(itered_data.value.clone()),
                    on_dot: true,
                    chain: vec![],
                })
            } else if data.complete && utils::is_opearators(letter_char) && is_s_n {
                //itered_data.value = types::Types::Operators(types::OperatorType {
                //    first: Box::new(itered_data.value.clone()),
                //    operator_collect: letter_char.to_string(),
                //    collecting_operator: true,
                //    ..Default::default()
                //});
            } else {
                if letter_char != " " {
                    //TODO IS THIS SAFE ?
                    data.comma = false;
                }
                let mut will_be_itered = if data.collective.len() == 0 {
                    variable::VariableCollector::default()
                } else {
                    variable::VariableCollector {
                        value: *data.collective[data.collective.len() - 1].value.clone(),
                        ..variable::VariableCollector::default()
                    }
                };
                let itered_array_vector = Box::new(collect(
                    &mut will_be_itered,
                    letter_char.clone(),
                    next_char.to_string().clone(),
                    last_char.to_string().clone(),
                    mapper::defs::CursorPosition(0, 0),
                ));

                if let types::Types::Array(ref adata) = itered_array_vector.itered_data.value {
                    if adata.complete {
                        data.child_start = false;
                    }
                }

                let itered_entry = match itered_array_vector.itered_data.value {
                    types::Types::Number(match_data) => types::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Number(match_data)),
                    },
                    types::Types::Double(match_data) => types::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Double(match_data)),
                    },
                    //types::Types::Operators(match_data) => types::ArrayEntry {
                    //    value_complete: false,
                    //    value: Box::new(types::Types::Operators(match_data)),
                    //},
                    types::Types::Bool(match_data) => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Bool(match_data)),
                    },
                    types::Types::String(match_data) => types::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::String(match_data)),
                    },
                    types::Types::Collective(_) => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Refference(_) => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Array(match_data) => types::ArrayEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Array(match_data)),
                    },
                    types::Types::Function => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::FunctionCall(_) => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Void => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Null => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                };

                if itered_array_vector.errors.len() != 0 {
                    for returned_error in itered_array_vector.errors {
                        //errors.extend(itered_array_vector.errors);
                        let mut edited = returned_error;
                        edited.pos.range_start.0 += pos.0;
                        edited.pos.range_start.1 += pos.1;
                        edited.pos.range_end.0 += pos.0;
                        edited.pos.range_end.1 += pos.1;
                        errors.push(edited);
                    }
                }

                if data.collective.len() == 0 {
                    data.collective.push(itered_entry);
                } else {
                    data.collective[last_entry - 1] = itered_entry;
                }
            }

            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Function => CollectorResponse {
            itered_data: itered_data.clone(),
            errors,
        },
        types::Types::FunctionCall(data) => {
            let mut last_param = data.params.len();
            if last_param == 0 {
                data.params.push(types::FunctionCallParameter::default());
                last_param = data.params.len();
            }

            let is_s_n =
                if last_param != 0 && data.params[last_param - 1].value.is_string_non_complete() {
                    false
                } else {
                    true
                };

            if letter_char == "," && is_s_n && !data.params[last_param - 1].value.is_array() {
                if data.params[last_param - 1].value.is_complete() {
                    data.comma = true;
                    data.params.push(types::FunctionCallParameter::default())
                } else {
                    errors.push(error::Error {
                        debug_message: "Crusial".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone().skipChar(1),
                            range_end: pos.clone().skipChar(2),
                        },
                    });
                }
            } else if letter_char == ")" && is_s_n {
                if data.comma {
                    errors.push(error::Error {
                        debug_message: "Qere".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }],
                        ),
                        pos: mapper::defs::Cursor {
                            range_start: pos.clone().skipChar(1),
                            range_end: pos.clone().skipChar(2),
                        },
                    });
                } else {
                    if data.params[last_param - 1].value.is_complete() || true {
                        data.complete = true
                    } else {
                        errors.push(error::Error {
                            debug_message: "Freede".to_string(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char.to_string(),
                                }],
                            ),
                            pos: mapper::defs::Cursor {
                                range_start: pos.clone().skipChar(1),
                                range_end: pos.clone().skipChar(2),
                            },
                        });
                    }
                }
            } else {
                let mut last_param_value = variable::VariableCollector {
                    value: data.params[last_param - 1].value.clone(),
                    ..variable::VariableCollector::default()
                };

                data.comma = false;

                let itered_param_value = Box::new(collect(
                    &mut last_param_value,
                    letter_char.clone(),
                    next_char.to_string().clone(),
                    last_char.to_string().clone(),
                    mapper::defs::CursorPosition(0, 0),
                ));

                let itered_entry = match itered_param_value.itered_data.value.clone() {
                    types::Types::Number(match_data) => types::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Number(match_data)),
                    },
                    types::Types::Double(match_data) => types::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::Double(match_data)),
                    },
                    //types::Types::Operators(match_data) => types::ArrayEntry {
                    //    value_complete: false,
                    //    value: Box::new(types::Types::Operators(match_data)),
                    //},
                    types::Types::Bool(match_data) => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Bool(match_data)),
                    },
                    types::Types::String(match_data) => types::ArrayEntry {
                        value_complete: match_data.complete,
                        value: Box::new(types::Types::String(match_data)),
                    },
                    types::Types::Collective(_) => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Refference(_) => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Array(match_data) => types::ArrayEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Array(match_data)),
                    },
                    types::Types::Function => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::FunctionCall(_) => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Void => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                    types::Types::Null => types::ArrayEntry {
                        value_complete: true,
                        value: Box::new(types::Types::Null),
                    },
                };

                if itered_param_value.errors.len() != 0 {
                    for returned_error in itered_param_value.errors {
                        //errors.extend(itered_array_vector.errors);
                        let mut edited = returned_error;
                        edited.pos.range_start.0 += pos.0;
                        edited.pos.range_start.1 += pos.1;
                        edited.pos.range_end.0 += pos.0;
                        edited.pos.range_end.1 += pos.1;
                        errors.push(edited);
                    }
                }
                data.params[last_param - 1].value = itered_param_value.itered_data.value;
            }

            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
        types::Types::Void => CollectorResponse {
            itered_data: itered_data.clone(),
            errors,
        },
        types::Types::Null => {
            //let is_num = itered_data.raw_value.parse::<usize>().is_ok();
            if itered_data.raw_value == "" {
                if letter_char == "\"" || letter_char == "'" {
                    itered_data.value = types::Types::String(types::StringType {
                        quote_type: letter_char.to_string(),
                        ..Default::default()
                    })
                } else if (itered_data.raw_value.clone() + &letter_char)
                    .to_string()
                    .parse::<i32>()
                    .is_ok()
                {
                    itered_data.value = types::Types::Number(types::NumberType {
                        value: (itered_data.raw_value.clone() + &letter_char)
                            .parse::<usize>()
                            .unwrap(),
                        complete: false,
                    })
                } else if letter_char == "[" {
                    println!("Array Started");
                    itered_data.value = types::Types::Array(types::ArrayType {
                        layer_size: 0,
                        child_start: false,
                        complete: false,
                        comma: false,
                        collective: Vec::new(),
                    });
                } else if letter_char == "{" {
                    println!("Objective Started");
                    itered_data.value = types::Types::Collective(types::CollectiveType {
                        layer_size: 0,
                        collective: Vec::new(),
                    });
                } else if letter_char == "(" {
                    //itered_data.value = types::Types::Operators(types::OperatorType {
                    //    brace: true,
                    //    ..Default::default()
                    //});
                } else if letter_char != " " {
                    itered_data.raw_value += &letter_char;
                }
            } else if letter_char != " " {
                if letter_char == "(" {
                    let current_reliability = crate::utils::reliable_name_range(
                        crate::utils::ReliableNameRanges::VariableName,
                        itered_data.raw_value.clone(),
                    );
                    if current_reliability.reliable {
                        itered_data.value = types::Types::FunctionCall(types::FunctionCall {
                            name: itered_data.raw_value.to_string(),
                            name_pos: mapper::defs::Cursor {
                                range_start: mapper::defs::CursorPosition(
                                    pos.0,
                                    pos.1 - itered_data.raw_value.len() as i64,
                                ),
                                range_end: mapper::defs::CursorPosition(pos.0, pos.1 - 1),
                            },
                            ..Default::default()
                        });
                    } else {
                        errors.push(error::Error {
                            debug_message: "Wole".to_string(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: current_reliability.found.to_string(),
                                }],
                            ),
                            pos: mapper::defs::Cursor {
                                range_start: mapper::defs::CursorPosition(
                                    pos.0,
                                    (pos.1 - itered_data.raw_value.len() as i64)
                                        + current_reliability.at as i64,
                                ),
                                range_end: mapper::defs::CursorPosition(
                                    pos.0,
                                    ((pos.1 - itered_data.raw_value.len() as i64)
                                        + current_reliability.at as i64)
                                        + 1,
                                ),
                            },
                        });
                    }
                } else if next_char == ";" || next_char == " " {
                    if itered_data.raw_value == "false" || itered_data.raw_value == "true" {
                        itered_data.value = types::Types::Bool(types::BoolType {
                            value: if itered_data.raw_value == "true" {
                                true
                            } else {
                                false
                            },
                        })
                    } else if itered_data.raw_value.parse::<i32>().is_ok() {
                        itered_data.value = types::Types::Number(types::NumberType {
                            value: (itered_data.raw_value.clone() + &letter_char)
                                .parse::<usize>()
                                .unwrap(),
                            complete: false,
                        })
                    }
                }
                itered_data.raw_value += &letter_char;
            }
            CollectorResponse {
                itered_data: itered_data.clone(),
                errors,
            }
        }
    }
}
