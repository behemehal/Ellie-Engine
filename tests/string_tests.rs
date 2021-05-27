#[cfg(test)]
mod string_tests {

    #[test]
    fn string_collected_with_no_error() {
        let pos = ellie_core::defs::CursorPosition(0, 0);
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "
            \"test\"
        ";

        for (index, char) in code.chars().enumerate() {
            if char == '\n' || char == '\r' {
                continue;
            }

            let letter_char = &char.to_string();
            let last_char =
                &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
            let next_char =
                &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
            let itered = ellie_parser::processors::value_processor::collect_value(
                &mut emulated_collector_data,
                letter_char,
                next_char.to_string(),
                last_char.to_string(),
                pos,
                ellie_core::defs::ParserOptions::default(),
            );

            for error in itered.errors {
                syntax_errors.push(error);
            }
            emulated_collector_data = itered.itered_data;
        }
        assert_eq!(syntax_errors.len(), 0);
        assert!(emulated_collector_data.data.value.is_string());
        assert!(emulated_collector_data.data.value.is_type_complete());
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::String(x) if x.value == "test")
        );
    }

    #[test]
    fn string_prototype_collected() {
        let pos = ellie_core::defs::CursorPosition(0, 0);
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "
            \"test\".len
        ";

        for (index, char) in code.chars().enumerate() {
            if char == '\n' || char == '\r' {
                continue;
            }

            let letter_char = &char.to_string();
            let last_char =
                &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
            let next_char =
                &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
            let itered = ellie_parser::processors::value_processor::collect_value(
                &mut emulated_collector_data,
                letter_char,
                next_char.to_string(),
                last_char.to_string(),
                pos,
                ellie_core::defs::ParserOptions::default(),
            );

            for error in itered.errors {
                syntax_errors.push(error);
            }
            emulated_collector_data = itered.itered_data;
        }
        assert_eq!(syntax_errors.len(), 0);
        assert!(emulated_collector_data.data.value.is_type_complete());
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::String(x) if x.value == "test")
        );
    }
}
