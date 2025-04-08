use log::debug;

use crate::errors::ParseError;
use crate::evaluator::Evaluator;
use crate::utils::Parser;

impl Parser {
    pub fn new(input: String) -> Parser {
        Self {
            input: input.replace(" ", "")
        }
    }

    fn validate_parenthesis_usage(&self) -> Result<(), ParseError> {
        if self.input.contains(")(") {
            return Err(ParseError::SyntaxError("adjacent parentheses in expression".to_string()));
        }
        
        for i in 0..self.input.len().saturating_sub(1) {
            let chars: Vec<char> = self.input.chars().collect();
            if chars[i].is_digit(10) && chars[i+1] == '(' {
                return Err(ParseError::SyntaxError("digit followed by opening parenthesis in expression".to_string()));
            }
            if i > 0 && chars[i] == ')' && chars[i+1].is_digit(10) {
                return Err(ParseError::SyntaxError("closing parenthesis followed by digit in expression".to_string()));
            }
        }
        
        Ok(())
    }

    /// Parses an input expression, handling parentheses and delegating
    /// actual calculation to the Evaluator
    pub fn parse_input(&mut self) -> Result<String, ParseError> {
        if self.input.is_empty() {
            return Err(ParseError::EmptyInputPassed);
        }

        self.validate_parenthesis_usage()?;

        // Process parentheses from innermost to outermost
        loop {
            let start_parenthesis_idx = match self.input.rfind('(') {
                Some(r) => r + 1,
                None => {
                    if self.input.find(')').is_some() {
                        return Err(ParseError::UnopenedParenthesis);
                    } else {
                        break;
                    }
                }
            };
            
            let end_parenthesis_idx = match self.input[start_parenthesis_idx..].find(')').map(|idx| start_parenthesis_idx + idx) {
                Some(r) => r,
                None => return Err(ParseError::MissingClosingParenthesis)
            };
            
            let inner_expr = &self.input[start_parenthesis_idx..end_parenthesis_idx];
            
            let res = Evaluator::evaluate_expression(inner_expr)
                .map_err(|e| ParseError::ComputeOperationFailed(e.to_string()))?;
            
            self.input.replace_range(start_parenthesis_idx - 1..end_parenthesis_idx + 1, &res.to_string());
            debug!("self.input: {}", self.input);
        }

        // After handling all parentheses, evaluate the final expression
        let res = Evaluator::evaluate_expression(&self.input)?;
        Ok(res.to_string())
    }
}