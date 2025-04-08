#[cfg(test)]
mod tests {
    use crate::{calculator::Calculator, errors::ParseError, lexer::{Lexer, Token}};

    // Lexer tests
    #[test]
    fn test_lexer_basic_numbers() {
        let mut lexer = Lexer::new("42".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Number(42.0)]);
    }

    #[test]
    fn test_lexer_decimal_numbers() {
        let mut lexer = Lexer::new("3.14".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Number(3.14)]);
    }

    #[test]
    fn test_lexer_negative_numbers() {
        let mut lexer = Lexer::new("-7".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Number(-7.0)]);
    }

    #[test]
    fn test_lexer_operators() {
        let mut lexer = Lexer::new("1+2-3*4/5".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Minus,
            Token::Number(3.0),
            Token::Multiply,
            Token::Number(4.0),
            Token::Divide,
            Token::Number(5.0),
        ]);
    }

    #[test]
    fn test_lexer_parentheses() {
        let mut lexer = Lexer::new("(1+2)".to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::OpenParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::CloseParen,
        ]);
    }

    #[test]
    fn test_lexer_invalid_character() {
        let mut lexer = Lexer::new("1+a".to_string());
        let result = lexer.tokenize();
        assert!(matches!(result, Err(ParseError::InvalidCharacter(_))));
    }

    #[test]
    fn test_lexer_empty_input() {
        let mut lexer = Lexer::new("".to_string());
        let result = lexer.tokenize();
        assert!(matches!(result, Err(ParseError::EmptyInputPassed)));
    }

    #[test]
    fn test_lexer_multiple_decimal_points() {
        let mut lexer = Lexer::new("3.14.159".to_string());
        let result = lexer.tokenize();
        assert!(matches!(result, Err(ParseError::SyntaxError(_))));
    }

    // Calculator tests
    #[test]
    fn test_calculator_simple_addition() {
        let result = Calculator::calculate("2+3").unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_calculator_subtraction() {
        let result = Calculator::calculate("5-3").unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_calculator_multiplication() {
        let result = Calculator::calculate("2*3").unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_calculator_division() {
        let result = Calculator::calculate("6/2").unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_calculator_order_of_operations() {
        let result = Calculator::calculate("2+3*4").unwrap();
        assert_eq!(result, 14.0);
    }

    #[test]
    fn test_calculator_parentheses() {
        let result = Calculator::calculate("(2+3)*4").unwrap();
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_calculator_nested_parentheses() {
        let result = Calculator::calculate("2*(3+(4-1))").unwrap();
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_calculator_with_decimals() {
        let result = Calculator::calculate("2.5*2").unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_calculator_divide_by_zero() {
        let result = Calculator::calculate("5/0");
        assert!(result.is_err());
    }

    #[test]
    fn test_calculator_invalid_expression() {
        let result = Calculator::calculate("5*");
        assert!(result.is_err());
    }

    #[test]
    fn test_calculator_unbalanced_parentheses() {
        let result = Calculator::calculate("(2+3");
        assert!(result.is_err());
    }

    #[test]
    fn test_calculator_implicit_multiplication() {
        let result = Calculator::calculate("2(3)");
        assert!(result.is_err());
    }

    #[test]
    fn test_calculator_complex_expression() {
        let result = Calculator::calculate("2*(3+4)/(2-0.5)").unwrap();
        assert!((result - 9.333333).abs() < 0.000001);
    }
}