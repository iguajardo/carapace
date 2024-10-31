pub struct InputBuffer {
    pub buffer: String,
    pub buffer_length: usize,
    pub input_length: usize,
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
