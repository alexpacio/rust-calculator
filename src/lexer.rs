use crate::errors::{EvaluationError, ParseError};


/// Token represents the smallest units in our expression
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParen,
    CloseParen,
}

/// Lexer converts a string into a sequence of tokens
pub struct Lexer {
    input: String,
    position: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.replace(" ", ""),
            position: 0,
            tokens: Vec::new(),
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> {
        if self.input.is_empty() {
            return Err(ParseError::EmptyInputPassed);
        }
        
        let chars: Vec<char> = self.input.chars().collect();
        
        while self.position < chars.len() {
            let c = chars[self.position];
            
            match c {
                '0'..='9' | '.' => {
                    let number = self.read_number(&chars)?;
                    self.tokens.push(Token::Number(number));
                }
                '+' => {
                    self.tokens.push(Token::Plus);
                    self.position += 1;
                }
                '-' => {
                    // Check if it's a negative number (unary minus) or subtraction
                    if self.position == 0 || matches!(chars.get(self.position - 1), Some('(') | Some('+') | Some('-') | Some('*') | Some('/')) {
                        // Unary minus (negative number follows)
                        if let Some('0'..='9' | '.') = chars.get(self.position + 1) {
                            self.position += 1; // Skip the minus
                            let number = self.read_number(&chars)?;
                            self.tokens.push(Token::Number(-number));
                        } else {
                            return Err(ParseError::SyntaxError("Invalid use of minus sign".to_string()));
                        }
                    } else {
                        // Binary minus (subtraction)
                        self.tokens.push(Token::Minus);
                        self.position += 1;
                    }
                }
                '*' => {
                    self.tokens.push(Token::Multiply);
                    self.position += 1;
                }
                '/' => {
                    self.tokens.push(Token::Divide);
                    self.position += 1;
                }
                '(' => {
                    self.tokens.push(Token::OpenParen);
                    self.position += 1;
                }
                ')' => {
                    self.tokens.push(Token::CloseParen);
                    self.position += 1;
                }
                _ => {
                    return Err(ParseError::InvalidCharacter(c.to_string()));
                }
            }
        }
        
        self.validate_token_sequence()?;
        
        Ok(self.tokens.clone())
    }
    
    fn read_number(&mut self, chars: &[char]) -> Result<f64, ParseError> {
        let start = self.position;
        let mut has_decimal = false;
        
        while self.position < chars.len() {
            match chars[self.position] {
                '0'..='9' => {
                    self.position += 1;
                }
                '.' if !has_decimal => {
                    has_decimal = true;
                    self.position += 1;
                }
                '.' if has_decimal => {
                    return Err(ParseError::SyntaxError("Multiple decimal points in a number".to_string()));
                }
                _ => {
                    break;
                }
            }
        }
        
        let number_str = chars[start..self.position].iter().collect::<String>();
        
        number_str.parse::<f64>()
            .map_err(|_| ParseError::EvaluationError(
                EvaluationError::ParseNumberError { value: number_str }
            ))
    }
    
    fn validate_token_sequence(&self) -> Result<(), ParseError> {
        // Check for empty expression
        if self.tokens.is_empty() {
            return Err(ParseError::EmptyInputPassed);
        }
        
        // Check for balanced parentheses
        let mut paren_balance = 0;
        for token in &self.tokens {
            match token {
                Token::OpenParen => paren_balance += 1,
                Token::CloseParen => {
                    paren_balance -= 1;
                    if paren_balance < 0 {
                        return Err(ParseError::UnopenedParenthesis);
                    }
                }
                _ => {}
            }
        }
        
        if paren_balance > 0 {
            return Err(ParseError::MissingClosingParenthesis);
        }
        
        // Check for valid sequences
        for i in 0..self.tokens.len() {
            match &self.tokens[i] {
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    // Operator shouldn't be at the start or end
                    if i == 0 {
                        return Err(ParseError::SyntaxError("Expression starts with an operator".to_string()));
                    }
                    
                    if i == self.tokens.len() - 1 {
                        return Err(ParseError::SyntaxError("Expression ends with an operator".to_string()));
                    }
                    
                    // Check what follows an operator
                    match self.tokens.get(i + 1) {
                        Some(Token::Plus) | Some(Token::Minus) | Some(Token::Multiply) | Some(Token::Divide) => {
                            return Err(ParseError::SyntaxError("Two operators in a row".to_string()));
                        }
                        Some(Token::CloseParen) => {
                            return Err(ParseError::SyntaxError("Operator followed by closing parenthesis".to_string()));
                        }
                        _ => {}
                    }
                }
                Token::OpenParen => {
                    // Check what follows an open parenthesis
                    if i < self.tokens.len() - 1 {
                        match self.tokens.get(i + 1) {
                            Some(Token::Plus) | Some(Token::Multiply) | Some(Token::Divide) => {
                                return Err(ParseError::SyntaxError("Open parenthesis followed by an operator".to_string()));
                            }
                            Some(Token::CloseParen) => {
                                return Err(ParseError::SyntaxError("Empty parentheses".to_string()));
                            }
                            _ => {}
                        }
                    }
                }
                Token::CloseParen => {
                    // Check what precedes a close parenthesis
                    if i > 0 {
                        match self.tokens.get(i - 1) {
                            Some(Token::Plus) | Some(Token::Minus) | Some(Token::Multiply) | Some(Token::Divide) => {
                                return Err(ParseError::SyntaxError("Closing parenthesis preceded by an operator".to_string()));
                            }
                            Some(Token::OpenParen) => {
                                return Err(ParseError::SyntaxError("Empty parentheses".to_string()));
                            }
                            _ => {}
                        }
                    }
                    
                    // Check for implicit multiplication
                    if i < self.tokens.len() - 1 {
                        if let Token::Number(_) = self.tokens[i + 1] {
                            return Err(ParseError::SyntaxError("Closing parenthesis followed by a number (implicit multiplication)".to_string()));
                        }
                    }
                }
                Token::Number(_) => {
                    // Check for implicit multiplication
                    if i < self.tokens.len() - 1 {
                        if let Token::OpenParen = self.tokens[i + 1] {
                            return Err(ParseError::SyntaxError("Number followed by opening parenthesis (implicit multiplication)".to_string()));
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}