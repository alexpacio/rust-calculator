use log::debug;
use regex::Regex;

use crate::errors::EvaluationError;

/// The Evaluator handles the mathematical operations on expressions
pub struct Evaluator;

impl Evaluator {
    /// Evaluates a mathematical expression string without parentheses
    pub fn evaluate_expression(expression: &str) -> Result<f64, EvaluationError> {
        debug!("expression to be evaluated: {}", expression);
    
        // Use a mutable string to track modifications
        let mut current = expression.to_string();
    
        // Regex for multiplication and division
        let regex_mult_div = Regex::new(r"(-?\d+(?:\.\d+)?)\s*([*/])\s*(-?\d+(?:\.\d+)?)").unwrap();
    
        // Process multiplication and division
        Self::process_regex_loop(&mut current, &regex_mult_div, |left, op, right| match op {
            "*" => Ok(left * right),
            "/" => {
                if right == 0.0 {
                    Err(EvaluationError::DivideByZero {
                        left,
                        right,
                    })
                } else {
                    Ok(left / right)
                }
            }
            _ => Err(EvaluationError::InvalidOperation {
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
            _ => Err(EvaluationError::InvalidOperation {
                left,
                right,
                op: op.to_string(),
            }),
        })?;
    
        debug!("final result string: {:#?}", current);
    
        // Convert the final string result to f64
        current.parse().map_err(|_| EvaluationError::ParseNumberError { value: current })
    }

    // Helper function that iteratively processes an operation
    fn process_regex_loop<F>(
        current: &mut String,
        regex: &Regex,
        op_fn: F,
    ) -> Result<(), EvaluationError>
    where
        F: Fn(f64, &str, f64) -> Result<f64, EvaluationError>,
    {
        while regex.is_match(current) {
            let caps = regex.captures(current).unwrap();

            let left_str = &caps[1];
            let op = &caps[2];
            let right_str = &caps[3];

            let left_operand: f64 = left_str.parse().map_err(|_| EvaluationError::ParseNumberError {
                value: left_str.to_string(),
            })?;
            let right_operand: f64 = right_str.parse().map_err(|_| EvaluationError::ParseNumberError {
                value: right_str.to_string(),
            })?;

            let result = op_fn(left_operand, op, right_operand)?;
            if result.is_nan() || result.is_infinite() {
                return Err(EvaluationError::InvalidOperation {
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
}