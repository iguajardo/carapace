use std::io::{self, Write};

use input_buffer::InputBuffer;

mod input_buffer;

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        read_input(&mut input_buffer.buffer);
    }
}

fn print_prompt() {
    print!("db > ");
}

fn read_input(input: &mut String) {
    io::stdout().flush().unwrap(); // ensure prompt is printed before reading line
    io::stdin().read_line(input).expect("Error trying to read the input.");

    println!("You entered: {}", input);
    input.clear();
}
