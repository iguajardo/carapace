use std::io::{self, Write};
use std::process;

use input_buffer::InputBuffer;

mod input_buffer;

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        // todo: make functional or keep it like this?
        // it blocks the thread until a line is entered.
        read_input(&mut input_buffer);

        match input_buffer.buffer.trim() {
            "exit" => process::exit(0),
            "" => println!("No command entered!"),
            other => println!("Unknown command: '{}'", other),
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap(); // ensure prompt is printed before reading line
}

fn read_input(input: &mut InputBuffer) {
    input.buffer.clear();
    let read_bytes = io::stdin()
        .read_line(&mut input.buffer)
        .expect("Error trying to read the input.");
    input.input_length = read_bytes - 1;
}
