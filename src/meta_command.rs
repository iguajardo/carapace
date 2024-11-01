use std::process;

use crate::input_buffer::InputBuffer;

#[repr(i32)]
pub enum MetaCommandResult {
    Success = 0,
    UnrecognizedCommand = 1,
}

pub fn do_meta_command(input_buffer: &InputBuffer) -> MetaCommandResult {
    let trimmed = input_buffer.buffer.trim();
    if trimmed == ".exit" {
        process::exit(0);
    } else {
        MetaCommandResult::UnrecognizedCommand
    }
}
