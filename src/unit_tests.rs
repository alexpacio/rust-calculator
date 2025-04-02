#[cfg(test)]
pub mod tests {
    use crate::{errors::{CalculationError, ParseError}, utils::{validate_char, ArithmeticOperationSign, CharMeaning, Parser}};

    // ---------- Tests for validate_char ----------

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

    // ---------- Tests for parse_expression (without parentheses) ----------

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
        // Expected: 2 + (3 * 4) = 2 + 12 = 14
        let expr = "2+3*4".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 14.0);
    }

    #[test]
    fn test_parse_expression_unary_minus() {
        // Unary minus at the beginning should work correctly.
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
        // 2*3*4 should be evaluated as (2*3)*4 = 24
        let expr = "2*3*4".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_parse_expression_complex_order() {
        // 2 + 3*4 - 5/2 = 2 + 12 - 2.5 = 11.5
        let expr = "2+3*4-5/2".to_string();
        let result = Parser::parse_expression(&expr).unwrap();
        assert!((result - 11.5).abs() < 1e-9);
    }

    #[test]
    fn test_parse_expression_parse_error() {
        // Expression contains invalid number format "2a+3"
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

    // ---------- Tests for parse_input (with parentheses) ----------

    #[test]
    fn test_parse_input_simple() {
        let mut parser = Parser::new("2+3".to_string());
        let result = parser.parse_input().unwrap();
        // The result is converted to string via to_string() on the f64 value.
        assert_eq!(result, "5");
    }

    #[test]
    fn test_parse_input_with_parentheses() {
        let mut parser = Parser::new("2*(3+4)".to_string());
        let result = parser.parse_input().unwrap();
        // 2*(3+4) = 2*7 = 14
        assert_eq!(result, "14");
    }

    #[test]
    fn test_parse_input_nested_parentheses() {
        let mut parser = Parser::new("2*(3+(4-1))".to_string());
        let result = parser.parse_input().unwrap();
        // Inner: (4-1) = 3, then (3+3) = 6, then 2*6 = 12
        assert_eq!(result, "12");
    }

    #[test]
    fn test_parse_input_extra_closing_parenthesis() {
        // This input has a closing parenthesis without an opening one.
        let mut parser = Parser::new("2+3)".to_string());
        let result = parser.parse_input();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::UnopenedParenthesis));
    }

    #[test]
    fn test_parse_input_missing_closing_parenthesis() {
        // This input is missing a closing parenthesis.
        let mut parser = Parser::new("2*(3+4".to_string());
        // Expecting a panic due to a missing closing parenthesis.
        let result = parser.parse_input();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::MissingClosingParenthesis));
    }

    #[test]
    fn test_parse_input_whitespace_handling() {
        // All whitespace should be ignored.
        let mut parser = Parser::new("   2 +   3 * 4   ".to_string());
        let result = parser.parse_input().unwrap();
        // Expected: 2 + (3*4) = 14
        assert_eq!(result, "14");
    }
}
