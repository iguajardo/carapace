use std::process;

#[repr(i32)]
pub enum MetaCommandResult {
    Success = 0,
    UnrecognizedCommand = 1,
}

pub fn do_meta_command(input: &str) -> MetaCommandResult {
    if input == ".exit" {
        process::exit(0);
    } else {
        MetaCommandResult::UnrecognizedCommand
    }
}
