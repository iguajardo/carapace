use std::io::{self, Write};

mod input_buffer;
mod meta_command;
mod prepare;

fn main() {
    let mut input_buffer = input_buffer::InputBuffer::new();

    loop {
        print_prompt();
        // todo: make functional or keep it like this?
        // it blocks the thread until a line is entered.
        read_input(&mut input_buffer);
        let ch = input_buffer.buffer.chars().nth(0).unwrap();
        if ch == '.' {
            match meta_command::do_meta_command(&input_buffer) {
                meta_command::MetaCommandResult::Success => {
                    continue;
                }
                meta_command::MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command '{}'", input_buffer.buffer.trim());
                    continue;
                }
            }
        }

        match input_buffer.buffer.trim() {
            "" => println!("No command entered!"),
            other => println!("Unknown command: '{}'", other),
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
