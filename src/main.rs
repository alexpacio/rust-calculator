use std::{io::{self, BufRead}, process, sync::mpsc::{self, Receiver}, thread};

use calculator::Calculator;

mod errors;
mod parser;
mod calculator;
mod lexer;
mod unit_tests;

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
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if input.trim().is_empty() {
                    continue;
                }
                
                match Calculator::calculate(&input) {
                    Ok(result) => println!("Result: {}", result),
                    Err(error) => eprintln!("Error: {}", error)
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}