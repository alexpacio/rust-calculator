#[cfg(test)]
pub mod tests {
    use crate::{errors::{CalculationError, ParseError}, utils::{validate_char, ArithmeticOperationSign, CharMeaning, Parser}};

    // Tests for validate_char

    #[test]
    fn test_validate_digit() {
        let result = validate_char(&'3');
        assert!(matches!(result, Ok(CharMeaning::Number)));
    }

    #[test]
    fn test_validate_whitespace() {
        let result = validate_char(&' ');
        assert!(matches!(result, Ok(CharMeaning::Whitespace)));
    }

    #[test]
    fn test_validate_multiply_sign() {
        let result = validate_char(&'*');
        assert!(matches!(result, Ok(CharMeaning::Sign(ArithmeticOperationSign::Multiply))));
    }

    #[test]
    fn test_validate_divide_sign() {
        let result = validate_char(&'/');
        assert!(matches!(result, Ok(CharMeaning::Sign(ArithmeticOperationSign::Divide))));
    }

    #[test]
    fn test_validate_add_sign() {
        let result = validate_char(&'+');
        assert!(matches!(result, Ok(CharMeaning::Sign(ArithmeticOperationSign::Add))));
    }

    #[test]
    fn test_validate_subtract_sign() {
        let result = validate_char(&'-');
        assert!(matches!(result, Ok(CharMeaning::Sign(ArithmeticOperationSign::Subtract))));
    }

    #[test]
    fn test_validate_parenthesis() {
        let left_paren = validate_char(&'(');
        let right_paren = validate_char(&')');
        assert!(matches!(left_paren, Ok(CharMeaning::Parenthesis)));
        assert!(matches!(right_paren, Ok(CharMeaning::Parenthesis)));
    }

    #[test]
    fn test_validate_invalid_char() {
        let result = validate_char(&'a');
        assert!(result.is_err());
    }

    // Tests for parse_expression (without parentheses)

    #[test]
    fn test_parse_expression_addition() {
        let expr = "2+3".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_parse_expression_subtraction() {
        let expr = "5-3".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_parse_expression_multiplication() {
        let expr = "2*3".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_parse_expression_division() {
        let expr = "6/2".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_parse_expression_precedence() {
        let expr = "2+3*4".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 14.0);
    }

    #[test]
    fn test_parse_expression_unary_minus() {
        let expr = "-3+5".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_parse_expression_decimals() {
        let expr = "2.5*2".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_parse_expression_multiple_operations() {
        let expr = "2*3*4".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_parse_expression_complex_order() {
        let expr = "2+3*4-5/2".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert!((result - 11.5).abs() < 1e-9);
    }

    #[test]
    fn test_parse_expression_parse_error() {
        let expr = "2a+3".to_string();
        let result = Parser::parse_expression(&expr);
        match result {
            Err(CalculationError::ParseError { value }) => {
                assert_eq!(value, "2a+3");
            },
            _ => panic!("Expected a ParseError"),
        }
    }

    #[test]
    fn test_parse_expression_divide_by_zero() {
        let expr = "4/0".to_string();
        let result = Parser::parse_expression(&expr);
        match result {
            Err(CalculationError::DivideByZero { left, right }) => {
                assert_eq!(left, 4.0);
                assert_eq!(right, 0.0);
            },
            _ => panic!("Expected a DivideByZero error"),
        }
    }

    #[test]
    fn test_parse_expression_empty_string() {
        let expr = "".to_string();
        let result = Parser::parse_expression(&expr);
        match result {
            Err(CalculationError::ParseError { value }) => {
                assert_eq!(value, "");
            },
            _ => panic!("Expected a ParseError for empty input"),
        }
    }

    // Tests for parse_input (with parentheses)

    #[test]
    fn test_parse_input_simple() {
        let mut parser = Parser::new("2+3".to_string());
        let result = parser.parse_input().unwrap();
        assert_eq!(result, "5");
    }

    #[test]
    fn test_parse_input_with_parentheses() {
        let mut parser = Parser::new("2*(3+4)".to_string());
        let result = parser.parse_input().unwrap();
        assert_eq!(result, "14");
    }

    #[test]
    fn test_parse_input_nested_parentheses() {
        let mut parser = Parser::new("2*(3+(4-1))".to_string());
        let result = parser.parse_input().unwrap();
        assert_eq!(result, "12");
    }

    #[test]
    fn test_parse_input_extra_closing_parenthesis() {
        let mut parser = Parser::new("2+3)".to_string());
        let result = parser.parse_input();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::UnopenedParenthesis));
    }

    #[test]
    fn test_parse_input_missing_closing_parenthesis() {
        let mut parser = Parser::new("2*(3+4".to_string());
        let result = parser.parse_input();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::MissingClosingParenthesis));
    }

    #[test]
    fn test_parse_input_digit_directly_preceding_parenthesis() {
        let mut parser = Parser::new("2352323(53)".to_string());
        let result = parser.parse_input();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::SyntaxError(_)));
    }

    #[test]
    fn test_parse_input_whitespace_handling() {
        let mut parser = Parser::new("   2 +   3 * 4   ".to_string());
        let result = parser.parse_input().unwrap();
        assert_eq!(result, "14");
    }
}
