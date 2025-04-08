use std::{io::{self, BufRead}, process, sync::mpsc::{self, Receiver}, thread};

mod errors;
mod utils;
mod parser;
mod evaluator;
mod unit_tests;

use errors::ParseError;
use utils::{validate_char, Parser};

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

fn main() {
    println!("Rust Calculator");
    println!("Enter expressions (e.g., '2 + 3 * (4 - 1)') to get the result. Press Ctrl-C to quit");

    let term_event_listener_recv = ctrl_channel().expect("Failed to setup the sigterm signal handler");

    let _ = thread::spawn(move || {
        if let Ok(()) = term_event_listener_recv.recv() {
            process::exit(0);
        }
    });

    let stdin = io::stdin();
    'main_loop: for line in stdin.lock().lines() {
        match line {
            Ok(input) => {                
                let mut validation_error: Option<ParseError> = None;
                'char_loop: for c in input.chars() {
                    match validate_char(&c) {
                        Err(e) => {
                            validation_error = Some(e);
                            break 'char_loop;
                        },
                        _ => ()
                    }
                }
                if let Some(error) = validation_error { 
                    eprintln!("Validation error: {}", error);
                    continue 'main_loop;
                };

                let mut parser = Parser::new(input);
                match parser.parse_input() {
                    Ok(res) => println!("Result: {}", res),
                    Err(e) => eprintln!("Error: {}", e)
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break 'main_loop;
            }
        }
    }
}