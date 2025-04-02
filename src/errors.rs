use std::{error::Error, fmt};

// Input string parsing errors
#[derive(Debug)]
pub enum ParseError {
    UnopenedParenthesis,
    MissingClosingParenthesis,
    ComputeOperationFailed(String),
    InvalidCharacter(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnopenedParenthesis => write!(f, "Input validation error: a parenthesis hasn't been opened while a closing one exists."),
            ParseError::MissingClosingParenthesis => write!(f, "Missing parenthesis closure"),
            ParseError::ComputeOperationFailed(msg) => write!(f, "Compute operation failed: {}", msg),
            ParseError::InvalidCharacter(c) => write!(f, "Compute operation failed: {}", c),
        }
    }
}

impl Error for ParseError {}

// Calculation errors

#[derive(Debug)]
pub enum CalculationError {
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