use std::io::{self, Write};
use std::process;

use input_buffer::InputBuffer;

mod input_buffer;

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        read_input(&mut input_buffer.buffer);

        match input_buffer.buffer.trim() {
            "exit" => {
                process::exit(0);
            }
            "" => println!("No command entered!"),
            other => println!("Unknown command: {}", other),
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap(); // ensure prompt is printed before reading line
}

fn read_input(input: &mut String) {
    input.clear();
    io::stdin()
        .read_line(input)
        .expect("Error trying to read the input.");
}
