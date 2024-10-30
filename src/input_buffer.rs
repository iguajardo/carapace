pub struct InputBuffer {
    pub buffer: String,
    buffer_length: i32,
    input_length: i32,
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer {
            buffer: String::new(),
            buffer_length: 0,
            input_length: 0,
        } 
    }
}
