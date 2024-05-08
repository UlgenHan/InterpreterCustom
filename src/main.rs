mod lib;

use lib::Interpreter::interpreter::Interpreter;
use std::env;
use std::fs;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if the argument count is correct
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        return;
    }

    // Get the file path from the arguments
    let file_path = &args[1];

    // Read the file contents
    let code = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    // Create an interpreter and interpret the code
    let interpreter = Interpreter::new(&code);
    if let Err(err) = interpreter.interpret() {
        eprintln!("Error interpreting code: {}", err);
    }
}

