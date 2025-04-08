use crate::{errors::{EvaluationError, ParseError}, lexer::Token};

/// Parser implements a recursive descent parser
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
    
    pub fn parse(&mut self) -> Result<f64, ParseError> {
        let result = self.parse_expression()?;
        
        // Make sure we consumed all tokens
        if self.position < self.tokens.len() {
            return Err(ParseError::SyntaxError("Unexpected tokens at the end of expression".to_string()));
        }
        
        Ok(result)
    }
    
    fn parse_expression(&mut self) -> Result<f64, ParseError> {
        self.parse_addition_subtraction()
    }
    
    fn parse_addition_subtraction(&mut self) -> Result<f64, ParseError> {
        let mut left = self.parse_multiplication_division()?;
        
        while self.position < self.tokens.len() {
            if let Some(token) = self.tokens.get(self.position) {
                match token {
                    Token::Plus => {
                        self.position += 1;
                        let right = self.parse_multiplication_division()?;
                        left += right;
                        
                        // Check for NaN or infinity
                        if left.is_nan() || left.is_infinite() {
                            return Err(ParseError::ComputeOperationFailed(
                                format!("Invalid operation: addition resulted in NaN or infinity")
                            ));
                        }
                    }
                    Token::Minus => {
                        self.position += 1;
                        let right = self.parse_multiplication_division()?;
                        left -= right;
                        
                        // Check for NaN or infinity
                        if left.is_nan() || left.is_infinite() {
                            return Err(ParseError::ComputeOperationFailed(
                                format!("Invalid operation: subtraction resulted in NaN or infinity")
                            ));
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
        
        Ok(left)
    }
    
    fn parse_multiplication_division(&mut self) -> Result<f64, ParseError> {
        let mut left = self.parse_primary()?;
        
        while self.position < self.tokens.len() {
            if let Some(token) = self.tokens.get(self.position) {
                match token {
                    Token::Multiply => {
                        self.position += 1;
                        let right = self.parse_primary()?;
                        left *= right;
                        
                        // Check for NaN or infinity
                        if left.is_nan() || left.is_infinite() {
                            return Err(ParseError::ComputeOperationFailed(
                                format!("Invalid operation: {} * {} resulted in NaN or infinity", left, right)
                            ));
                        }
                    }
                    Token::Divide => {
                        self.position += 1;
                        let right = self.parse_primary()?;
                        
                        if right == 0.0 {
                            return Err(ParseError::EvaluationError(
                                EvaluationError::DivideByZero { left, right }
                            ));
                        }
                        
                        left /= right;
                        
                        // Check for NaN or infinity
                        if left.is_nan() || left.is_infinite() {
                            return Err(ParseError::EvaluationError(
                                EvaluationError::InvalidOperation { 
                                    left: left, 
                                    right: right, 
                                    op: "/".to_string() 
                                }
                            ));
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
        
        Ok(left)
    }
    
    fn parse_primary(&mut self) -> Result<f64, ParseError> {
        if let Some(token) = self.tokens.get(self.position) {
            match token {
                Token::Number(value) => {
                    self.position += 1;
                    Ok(*value)
                }
                Token::OpenParen => {
                    self.position += 1;
                    let expression_value = self.parse_expression()?;
                    
                    // Ensure closing parenthesis
                    if self.position >= self.tokens.len() || !matches!(self.tokens.get(self.position), Some(Token::CloseParen)) {
                        return Err(ParseError::MissingClosingParenthesis);
                    }
                    
                    self.position += 1;
                    Ok(expression_value)
                }
                _ => {
                    Err(ParseError::SyntaxError("Expected a number or opening parenthesis".to_string()))
                }
            }
        } else {
            Err(ParseError::SyntaxError("Unexpected end of input".to_string()))
        }
    }
}