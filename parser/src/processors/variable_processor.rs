use crate::parser;
use crate::processors;
use crate::syntax::variable;
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct CollectorResponse {
    parser: parser::Parser,
    data: variable::VariableCollector,
}

pub fn collect_variable_value(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    options: defs::ParserOptions,
) {
    if let parser::Collecting::Variable(ref mut variabledata) = parser.current {
        if !variabledata.named {
            if letter_char == ":" {
                if variabledata.data.name.is_empty() {
                    errors.push(error::Error {
                        debug_message: "e2b7db512d6d3802b8192b736239bf0c"
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
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                } else {
                    if variabledata.data.dynamic {
                        //TODO REMOVE THIS
                        errors.push(error::Error {
                            debug_message: "9556d21bbef5c0d7a0863e5cf1fd0a9e"
                                .to_string(),
                            title: error::errorList::error_s11.title.clone(),
                            code: error::errorList::error_s11.code,
                            message: error::errorList::error_s11.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s11.message.clone(),
                            ),
                            pos: defs::Cursor {
                                range_start: parser.pos.clone().skipChar(1),
                                range_end: parser.pos.clone().skipChar(2),
                            },
                        });
                    }
                    variabledata.named = true;
                }
            } else if letter_char == ";" && variabledata.data.dynamic {
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                #[cfg(feature = "std")]
                std::println!(
                    "{}[ParserWarning]{}: WORKING BLIND, Read more: {}https://github.com/behemehal/Ellie-Language/issues/2{}",
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Yellow),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Cyan),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                );

                if !variabledata.data.dynamic {
                    errors.push(error::Error {
                        debug_message: "44bcf70e8e297896f6850b3d62dba97f"
                            .to_string(),
                        title: error::errorList::error_s8.title.clone(),
                        code: error::errorList::error_s8.code,
                        message: error::errorList::error_s8.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s8.message.clone(),
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                } else if variabledata.data.name.is_empty() {
                    errors.push(error::Error {
                        debug_message: "73279dfa4bb187ca8c42f48bc4cd95c4"
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
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                } else {
                    variabledata.named = true;
                }
            } else {
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    if last_char == " " && !variabledata.data.name.is_empty() {
                        errors.push(error::Error {
                            debug_message: "00abf5545d2f089ca47899251de22a2e"
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
                            pos: defs::Cursor {
                                range_start: parser.pos,
                                range_end: parser.pos.clone().skipChar(1),
                            },
                        });
                    } else {
                        variabledata.data.name = variabledata.data.name.clone() + letter_char;
                    }
                } else if letter_char != " "
                    && (letter_char != ":" || letter_char != "=" || letter_char != ";")
                    && (last_char == " " || !variabledata.data.name.is_empty())
                {
                    errors.push(error::Error {
                        debug_message: "06fc978c390e552b678eb3c0b29b9e0e"
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
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                }
            }
        } else if !variabledata.typed && !variabledata.data.dynamic {
            if letter_char == ";" {
                #[cfg(feature = "std")]
                std::println!(
                    "{}[ParserWarning]{}: WORKING BLIND, Read more: {}https://github.com/behemehal/Ellie-Language/issues/2{}",
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Yellow),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Cyan),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                );
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                if !variabledata.data.rtype.is_definer_complete() {
                    errors.push(error::Error {
                        debug_message: "57b0c03670bda2d42a364d1380bf97e6"
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
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                } else {
                    #[cfg(feature = "std")]
                    std::println!(
                        "{}[ParserWarning]{}: WORKING BLIND, Read more: {}https://github.com/behemehal/Ellie-Language/issues/2{}",
                        utils::terminal_colors::get_color(utils::terminal_colors::Colors::Yellow),
                        utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                        utils::terminal_colors::get_color(utils::terminal_colors::Colors::Cyan),
                        utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                    );
                    variabledata.typed = true;
                }
            } else {
                if variabledata.data.type_pos.range_start.0 == 0 && variabledata.data.type_pos.range_start.1 == 0 && letter_char != " " {
                    variabledata.data.type_pos.range_start = parser.pos;
                }
                processors::definer_processor::collect_definer(
                    &mut variabledata.data.rtype,
                    errors,
                    letter_char.to_string(),
                    parser.pos,
                    next_char,
                    last_char,
                    options,
                );
                variabledata.data.type_pos.range_end = parser.pos;
            }
        } else if letter_char == ";" {
            variabledata.data.value_pos.range_end = parser.pos;
            if let parser::Collecting::Variable(ref mut collected) = parser.current {
                if collected.data.value.is_type_complete() {
                    collected.data.pos.range_end = parser.pos;
                    collected.data.value_pos.range_end = parser.pos;

                    if collected.data.rtype.raw_name() != collected.data.value.get_type() {
                        //We should resolve inner value

                        if collected.data.dynamic {
                            #[cfg(feature = "std")]
                            std::println!(
                                "{}[ParserError]{}: This is a error please report at: {}https://github.com/behemehal/Ellie-Language/issues/new?title=ParserError-{}+Dynamic+Variable+Not+Handled+Correctly&labels=bug,parser{}",
                                utils::terminal_colors::get_color(utils::terminal_colors::Colors::Red),
                                utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                                utils::terminal_colors::get_color(utils::terminal_colors::Colors::Cyan),
                                collected.data.value.get_type(),
                                utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                            );
                            
                            #[cfg(feature = "std")]
                            std::process::exit(0);
                        }

                        errors.push(error::Error {
                            debug_message: "7047e122b063916e8fad46e3394ed1b6"
                                .to_string(),
                            title: error::errorList::error_s3.title.clone(),
                            code: error::errorList::error_s3.code,
                            message: error::errorList::error_s3.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s3.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "token1".to_string(),
                                        value: collected.data.rtype.raw_name(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_string(),
                                        value: collected.data.value.get_type()
                                    },
                                ],
                            ),
                            pos: collected.data.value_pos
                        });
                    }
                    parser.collected.push(parser.current.clone());
                    parser.current = parser::Collecting::None;
                } else {
                    std::println!("{:#?}", collected);
                    errors.push(error::Error {
                        debug_message: "1956fe42665d967b58bdd830385c2e9f"
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
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                }
            }
        } else {
            if variabledata.data.value_pos.range_start.0 == 0 && variabledata.data.value_pos.range_start.1 == 0 && letter_char != " " {
                variabledata.data.value_pos.range_start = parser.pos;
            }
            let mut cd = variabledata.clone();
            let collected = processors::value_processor::collect_value(
                &mut cd,
                letter_char,
                next_char,
                last_char,
                parser.pos,
                options,
            );
            for i in collected.errors {
                errors.push(i)
            }
            variabledata.data.value_pos.range_end = parser.pos;
            parser.current = parser::Collecting::Variable(collected.itered_data);
        }
    }
}






