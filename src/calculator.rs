use crate::{errors::ParseError, lexer::Lexer, parser::Parser};

pub struct Calculator;

impl Calculator {
    pub fn calculate(input: &str) -> Result<f64, ParseError> {
        // Step 1: Tokenize the input
        let mut lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize()?;
        
        // Step 2: Parse and evaluate the tokens
        let mut parser = Parser::new(tokens);
        parser.parse()
    }
}