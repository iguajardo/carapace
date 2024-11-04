use std::io::{self, Write};

use prepare::{prepare_statement, Statement, PrepareResult};

mod input_buffer;
mod meta_command;
mod prepare;

fn main() {
    let mut input_buffer = input_buffer::InputBuffer::new();

    loop {
        print_prompt();
        read_input(&mut input_buffer);

        let trimmed_input = input_buffer.buffer.trim();

        if trimmed_input.starts_with('.') {
            match meta_command::do_meta_command(trimmed_input) {
                meta_command::MetaCommandResult::Success => continue,
                meta_command::MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command '{}'", trimmed_input);
                    continue;
                }
            }
        }

        let mut statement = Statement::new(); // todo: add new constructor
        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::Success => {
                break;
            }
            PrepareResult::UnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'", input_buffer.buffer);
                continue;
            }
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap(); // ensure prompt is printed before reading line
}

fn read_input(input: &mut input_buffer::InputBuffer) {
    input.buffer.clear();
    let read_bytes = io::stdin()
        .read_line(&mut input.buffer)
        .expect("Error trying to read the input.");
    input.input_length = read_bytes - 1;
}
