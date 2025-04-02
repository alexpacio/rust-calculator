use std::{error::Error, fmt, io::{self, BufRead}};
use log::debug;
use regex::Regex;

#[derive(Debug)]
enum CalculationError {
    DivideByZero { left: f64, right: f64 },
    ParseError { value: String },
    InvalidOperation { left: f64, right: f64, op: String },
}

impl fmt::Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculationError::DivideByZero { left, right } => 
                write!(f, "Division by zero: {} / {}", left, right),
            CalculationError::ParseError { value } => 
                write!(f, "Failed to parse number: '{}'", value),
            CalculationError::InvalidOperation { left, right, op } => 
                write!(f, "Invalid operation: {} {} {} resulted in NaN or infinity", left, op, right),
        }
    }
}

impl Error for CalculationError {}

enum ArithmeticOperationSign {
    Multiply,
    Divide,
    Add,
    Subtract
}

enum CharMeaning {
    Number,
    Sign(ArithmeticOperationSign),
    Whitespace,
    Parenthesis
}

struct Parser {
    input: String
}

fn validate_char(char: &char) -> Result<CharMeaning, Box<dyn Error>> {
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
        _ => return Err(format!("Invalid character detected: {}", char).into())
    }
}

impl Parser {
    fn new(input: String) -> Parser {
        Self {
            input: input.replace(" ", "")
        }
    }

    fn parse_expression(expression: &String) -> Result<f64, CalculationError> {
        debug!("expression to be processed: {}", expression);
        // Create a regex pattern to match multiplication and division operations
        let regex_mult_div = Regex::new(r"(-?\d+(?:\.\d+)?)\s*([*/])\s*(-?\d+(?:\.\d+)?)").unwrap();
        
        // Use a mutable string to track modifications
        let mut current = expression.to_string();
        
        // Process the expression iteratively to handle aborting
        while regex_mult_div.is_match(&current) {
            let caps = regex_mult_div.captures(&current).unwrap();
            
            let left_str = &caps[1];
            let op = &caps[2];
            let right_str = &caps[3];
            
            let left_operand: f64 = left_str.parse()
                .map_err(|_| CalculationError::ParseError { value: left_str.to_string() })?;
                
            let right_operand: f64 = right_str.parse()
                .map_err(|_| CalculationError::ParseError { value: right_str.to_string() })?;
            
            let result = match op {
                "*" => left_operand * right_operand,
                "/" => {
                    if right_operand == 0.0 {
                        return Err(CalculationError::DivideByZero { 
                            left: left_operand, 
                            right: right_operand 
                        });
                    }
                    left_operand / right_operand
                },
                _ => return Err(CalculationError::InvalidOperation { 
                    left: left_operand, 
                    right: right_operand, 
                    op: op.to_string() 
                }),
            };
            
            if result.is_nan() || result.is_infinite() {
                return Err(CalculationError::InvalidOperation { 
                    left: left_operand, 
                    right: right_operand, 
                    op: op.to_string() 
                });
            }
            
            // Replace all the matches to not repeat the exact same arithmetic operation if repeated
            current = regex_mult_div.replace_all(&current, format!("{}", result)).to_string();
        }

        debug!("current: {:#?}", current);

        let regex_add_subt = Regex::new(r"(-?\d+(?:\.\d+)?)\s*([+-])\s*(-?\d+(?:\.\d+)?)").unwrap();

        while regex_add_subt.is_match(&current) {
            let caps = regex_add_subt.captures(&current).unwrap();
            
            let left_str = &caps[1];
            let op = &caps[2];
            let right_str = &caps[3];
            
            let left_operand: f64 = left_str.parse()
                .map_err(|_| CalculationError::ParseError { value: left_str.to_string() })?;
                
            let right_operand: f64 = right_str.parse()
                .map_err(|_| CalculationError::ParseError { value: right_str.to_string() })?;
            
            let result = match op {
                "+" => left_operand + right_operand,
                "-" => left_operand - right_operand,
                _ => return Err(CalculationError::InvalidOperation { 
                    left: left_operand, 
                    right: right_operand, 
                    op: op.to_string() 
                }),
            };
            
            if result.is_nan() || result.is_infinite() {
                return Err(CalculationError::InvalidOperation { 
                    left: left_operand, 
                    right: right_operand, 
                    op: op.to_string() 
                });
            }
            
            // Replace all the matches to not repeat the exact same arithmetic operation if repeated
            current = regex_add_subt.replace_all(&current, format!("{}", result)).to_string();
        }

        debug!("result2: {:#?}", current);

        Ok(current.parse().map_err(|_| CalculationError::ParseError { value: current.to_string() })?)
    }

    fn parse_input(&mut self) -> Result<String, Box<dyn Error>> {
        // find parenthesis and iterate them from the deepest to the less important
        loop {
            let start_parenthesis_idx = match self.input.rfind('(') {
                Some(r) => r + 1,
                None => {
                    if self.input.find(')').is_some() {
                        return Err(format!("Input validation error: a parenthesis hasn't been opened while the closure exists.").into());
                    } else {
                        break;
                    }
                }
            };
            let end_parenthesis_idx = match self.input[start_parenthesis_idx..].find(')').map(|idx| start_parenthesis_idx + idx) {
                Some(r) => r,
                None => panic!("Missing parenthesis closure")
            };
            let str = self.input[start_parenthesis_idx..end_parenthesis_idx].to_string();
            let res = match Self::parse_expression(&str) {
                Ok(res) => res,
                Err(e) => return Err(format!("Compute operation failed: {}", e.to_string()).into())
            };
            self.input.replace_range(start_parenthesis_idx - 1..end_parenthesis_idx + 1, &res.to_string());
            println!("self.input: {}", self.input);
        }

        let res = match Self::parse_expression(&self.input) {
            Ok(res) => res,
            Err(e) => return Err(format!("Compute operation failed: {}", e.to_string()).into())
        };

        Ok(res.to_string())
    }
    
}


fn main() {
    println!("Rust Calculator");
    println!("Enter expressions (e.g., '2 + 3 * (4 - 1)') or 'exit' to quit");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if input.trim().to_lowercase() == "exit" {
                    break;
                }
                
                let mut validation_error = false;
                for c in input.chars() {
                    match validate_char(&c) {
                        Err(e) => {
                            eprintln!("Validation error: {}", e);
                            validation_error = true;
                            break;
                        },
                        _ => ()
                    }
                }
                if validation_error { continue };

                let mut parser = Parser::new(input);
                match parser.parse_input() {
                    Ok(res) => println!("Result: {}", res),
                    Err(e) => println!("Error: {}", e)
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                break;
            }
        }
    }
}
/* 
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
} */