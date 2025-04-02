use crate::errors::ParseError;

pub enum ArithmeticOperationSign {
    Multiply,
    Divide,
    Add,
    Subtract
}

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
        '*' => return Ok(CharMeaning::Sign(ArithmeticOperationSign::Multiply)),
        '/' => return Ok(CharMeaning::Sign(ArithmeticOperationSign::Divide)),
        '-' => return Ok(CharMeaning::Sign(ArithmeticOperationSign::Subtract)),
        '+' => return Ok(CharMeaning::Sign(ArithmeticOperationSign::Add)),
        '(' | ')' => return Ok(CharMeaning::Parenthesis),
        _ => return Err(ParseError::InvalidCharacter(char.to_string()))
    }
}