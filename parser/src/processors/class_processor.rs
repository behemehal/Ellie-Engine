use crate::parser;
#[allow(unused_imports)]
use crate::syntax::{class, function, types};
#[allow(unused_imports)]
use ellie_core::{defs, error, utils};

#[allow(unused_imports)]
use crate::alloc::string::{String, ToString};
#[allow(unused_imports)]
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_class(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    _options: defs::ParserOptions,
) {
    if let parser::Collecting::Class(ref mut classdata) = parser.current {
        if !classdata.name_collected {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable {
                if last_char == " " {
                    //class name is now typing
                    classdata.name_pos.range_start = parser.pos;
                }
                classdata.name_pos.range_end = parser.pos;
                classdata.data.name += letter_char;
            } else if letter_char == " " && !classdata.data.name.is_empty() {
                classdata.name_collected = true;
            } else if letter_char == "<" && !classdata.data.name.is_empty() {
                classdata.name_collected = true;
                classdata.generic_brace_open = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    debug_message: "".to_string(),
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
        } else if !classdata.generic_definings_collected {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if classdata.generic_brace_open {
                let mut last_entry = classdata.data.generic_definings.len();

                if current_reliability.reliable
                    && (last_char != " "
                        || last_entry == 0
                        || classdata.data.generic_definings[last_entry - 1]
                            .name
                            .is_empty())
                {
                    if last_entry == 0 {
                        classdata
                            .data
                            .generic_definings
                            .push(class::GenericDefining {
                                pos: defs::Cursor {
                                    range_start: parser.pos.clone().popChar(1),
                                    range_end: parser.pos.clone().skipChar(1),
                                },
                                ..Default::default()
                            });
                        last_entry = 1;
                    }

                    if classdata.data.generic_definings[last_entry - 1]
                        .name
                        .is_empty()
                    {
                        classdata.data.generic_definings[last_entry - 1]
                            .pos
                            .range_start = parser.pos;
                    };
                    classdata.data.generic_definings[last_entry - 1]
                        .pos
                        .range_end = parser.pos.clone().skipChar(1);
                    classdata.data.generic_definings[last_entry - 1].name += letter_char
                } else if letter_char == ">" {
                    if last_entry == 0
                        || classdata.data.generic_definings[last_entry - 1]
                            .name
                            .is_empty()
                    {
                        errors.push(error::Error {
                            debug_message: "".to_string(),
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
                        classdata.generic_brace_open = false;
                        classdata.generic_definings_collected = true;
                    }
                } else if letter_char == "," {
                    if last_entry == 0
                        || classdata.data.generic_definings[last_entry - 1]
                            .name
                            .is_empty()
                    {
                        errors.push(error::Error {
                            debug_message: "".to_string(),
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
                        classdata
                            .data
                            .generic_definings
                            .push(class::GenericDefining {
                                pos: defs::Cursor {
                                    range_start: parser.pos,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                    }
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "".to_string(),
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
            } else if letter_char == "<" {
                classdata.generic_brace_open = true;
            } else if letter_char == "{" {
                classdata.generic_definings_collected = true;
            } else {
                errors.push(error::Error {
                    debug_message: "".to_string(),
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
        } else if letter_char == "}" && classdata.brace_count == 0 {
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if letter_char == "{" {
                if classdata.collecting_code {
                    classdata.brace_count += 1;
                } else {
                    classdata.collecting_code = true;
                }
            } else if letter_char == "}" && classdata.brace_count > 0 {
                classdata.brace_count -= 1;
            }
            classdata.inside_code_string += letter_char;
            let mut child_parser = parser::Parser::new(
                classdata.inside_code_string.clone(),
                defs::ParserOptions {
                    parser_type: defs::ParserType::ClassParser,
                    ..Default::default()
                },
            );
            child_parser.pos = parser.pos;
            let mapped = child_parser.map();
            for i in mapped.syntax_errors {
                errors.push(i)
            }

            for item in mapped.items {
                match item {
                    parser::Collecting::Variable(e) => {
                        classdata.data.properties.push(e.data);
                    }
                    parser::Collecting::Function(e) => {
                        classdata.data.methods.push(e.data);
                    }
                    parser::Collecting::Constructor(e) => {
                        classdata.data.constructor = e.data;
                    }
                    parser::Collecting::Getter => {}
                    parser::Collecting::Setter => {}
                    _ => (),
                };
            }
        }
    }
    //parser.collected = [].to_vec();
    //parser.collected.push(parser.current.clone());
}
