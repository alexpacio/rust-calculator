use thiserror::Error;

// Input string parsing errors
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Input validation error: a parenthesis hasn't been opened while a closing one exists")]
    UnopenedParenthesis,
    
    #[error("Missing parenthesis closure")]
    MissingClosingParenthesis,
    
    #[error("Compute operation failed: {0}")]
    ComputeOperationFailed(String),
    
    #[error("Invalid character found: {0}")]
    InvalidCharacter(String),
    
    #[error("Empty input passed")]
    EmptyInputPassed,
    
    #[error("Expression syntax error: {0}")]
    SyntaxError(String),
    
    #[error("Evaluation error: {0}")]
    EvaluationError(#[from] EvaluationError),
}

// Calculation errors
#[derive(Error, Debug)]
pub enum EvaluationError {
    #[error("Division by zero: {left} / {right}")]
    DivideByZero { left: f64, right: f64 },
    
    #[error("Failed to parse number: '{value}'")]
    ParseNumberError { value: String },
    
    #[error("Invalid operation: {left} {op} {right} resulted in NaN or infinity")]
    InvalidOperation { left: f64, right: f64, op: String },
}