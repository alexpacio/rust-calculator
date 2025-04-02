use std::{io::{self, BufRead}, process, sync::mpsc::{self, Receiver}, thread};
mod errors;
mod utils;
mod parser;
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
    println!("Enter expressions (e.g., '2 + 3 * (4 - 1)') to get the result. Type 'exit' or Ctrl-C to quit");

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
                if input.trim().to_lowercase() == "exit" {
                    break 'main_loop;
                }
                
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
                if validation_error.is_some() { 
                    eprintln!("Validation error: {}", validation_error.unwrap().to_string());
                    continue 'main_loop;
                };

                let mut parser = Parser::new(input);
                match parser.parse_input() {
                    Ok(res) => println!("Result: {}", res),
                    Err(e) => println!("Error: {}", e)
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                break 'main_loop;
            }
        }
    }
}
