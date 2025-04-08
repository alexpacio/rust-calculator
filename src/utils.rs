use crate::errors::ParseError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArithmeticOperationSign {
    Multiply,
    Divide,
    Add,
    Subtract
}

#[derive(Debug)]
pub enum CharMeaning {
    Number,
    #[allow(dead_code)]
    Sign(ArithmeticOperationSign),
    Whitespace,
    Parenthesis
}

pub struct Parser {
    pub input: String
}

pub fn validate_char(char: &char) -> Result<CharMeaning, ParseError> {
    if char.is_digit(10) {
        return Ok(CharMeaning::Number)
    } else if char.is_whitespace() {
        return Ok(CharMeaning::Whitespace)
    }
    match char {
        '*' => Ok(CharMeaning::Sign(ArithmeticOperationSign::Multiply)),
        '/' => Ok(CharMeaning::Sign(ArithmeticOperationSign::Divide)),
        '-' => Ok(CharMeaning::Sign(ArithmeticOperationSign::Subtract)),
        '+' => Ok(CharMeaning::Sign(ArithmeticOperationSign::Add)),
        '(' | ')' => Ok(CharMeaning::Parenthesis),
        _ => Err(ParseError::InvalidCharacter(char.to_string()))
    }
}