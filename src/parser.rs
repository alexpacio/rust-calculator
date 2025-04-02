use log::debug;
use regex::Regex;

use crate::{errors::{CalculationError, ParseError}, utils::Parser};

impl Parser {
    pub fn new(input: String) -> Parser {
        Self {
            input: input.replace(" ", "")
        }
    }

    // Helper function that iteratively processes an operation
    pub fn process_regex_loop<F>(
        current: &mut String,
        regex: &Regex,
        op_fn: F,
    ) -> Result<(), CalculationError>
    where
        F: Fn(f64, &str, f64) -> Result<f64, CalculationError>,
    {
        while regex.is_match(current) {
            let caps = regex.captures(current).unwrap();

            let left_str = &caps[1];
            let op = &caps[2];
            let right_str = &caps[3];

            let left_operand: f64 = left_str.parse().map_err(|_| CalculationError::ParseError {
                value: left_str.to_string(),
            })?;
            let right_operand: f64 = right_str.parse().map_err(|_| CalculationError::ParseError {
                value: right_str.to_string(),
            })?;

            let result = op_fn(left_operand, op, right_operand)?;
            if result.is_nan() || result.is_infinite() {
                return Err(CalculationError::InvalidOperation {
                    left: left_operand,
                    right: right_operand,
                    op: op.to_string(),
                });
            }

            // Replace only the first occurrence
            *current = regex.replace(current, format!("{}", result)).to_string();
        }
        Ok(())
    }

    pub fn parse_expression(expression: &String) -> Result<f64, CalculationError> {
        debug!("expression to be processed: {}", expression);
    
        // Use a mutable string to track modifications
        let mut current = expression.to_string();
    
        // Regex for multiplication and division
        let regex_mult_div = Regex::new(r"(-?\d+(?:\.\d+)?)\s*([*/])\s*(-?\d+(?:\.\d+)?)").unwrap();
    
        // Process multiplication and division
        Self::process_regex_loop(&mut current, &regex_mult_div, |left, op, right| match op {
            "*" => Ok(left * right),
            "/" => {
                if right == 0.0 {
                    Err(CalculationError::DivideByZero {
                        left,
                        right,
                    })
                } else {
                    Ok(left / right)
                }
            }
            _ => Err(CalculationError::InvalidOperation {
                left,
                right,
                op: op.to_string(),
            }),
        })?;
    
        debug!("current after mult/div: {:#?}", current);
    
        // Regex for addition and subtraction
        let regex_add_subt = Regex::new(r"(-?\d+(?:\.\d+)?)\s*([+-])\s*(-?\d+(?:\.\d+)?)").unwrap();
    
        // Process addition and subtraction
        Self::process_regex_loop(&mut current, &regex_add_subt, |left, op, right| match op {
            "+" => Ok(left + right),
            "-" => Ok(left - right),
            _ => Err(CalculationError::InvalidOperation {
                left,
                right,
                op: op.to_string(),
            }),
        })?;
    
        debug!("final result string: {:#?}", current);
    
        // Convert the final string result to f64
        current.parse().map_err(|_| CalculationError::ParseError { value: current })
    }

    fn validate_parenthesis_usage(&self) -> Result<(), ParseError> {
        let implicit_mult_regex = Regex::new(r"(\d\()|(\)\d)|(\)\()").unwrap();
        
        if let Some(captures) = implicit_mult_regex.captures(&self.input) {
            let error_message = if captures.get(1).is_some() {
                format!("digit followed by opening parenthesis in expression")
            } else if captures.get(2).is_some() {
                format!("closing parenthesis followed by digit in expression")
            } else {
                format!("adjacent parentheses in expression")
            };
            
            return Err(ParseError::SyntaxError(error_message));
        }
        
        Ok(())
    }

    pub fn parse_input(&mut self) -> Result<String, ParseError> {
        // Find parenthesis and iterate starting from the top one until the one from the bottom of the stack is reached
        if self.input.len() == 0 {
            return Err(ParseError::EmptyInputPassed)
        }

        self.validate_parenthesis_usage()?;

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
            let str = self.input[start_parenthesis_idx..end_parenthesis_idx].to_string();
            let res = match Self::parse_expression(&str) {
                Ok(res) => res,
                Err(e) => return Err(ParseError::ComputeOperationFailed(e.to_string()))
            };
            self.input.replace_range(start_parenthesis_idx - 1..end_parenthesis_idx + 1, &res.to_string());
            debug!("self.input: {}", self.input);
        }

        let res = match Self::parse_expression(&self.input) {
            Ok(res) => res,
            Err(e) => return Err(ParseError::ComputeOperationFailed(e.to_string()))
        };

        Ok(res.to_string())
    }
}