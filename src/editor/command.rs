pub struct CommandLine {
    pub buffer: String,
}

impl CommandLine {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }
    
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn stringify(&self) -> Vec<&str> {
        self.buffer.split(" ").collect()
    }

    pub fn push(&mut self, char: char) {
        self.buffer.push(char);
    }

    pub fn pop(&mut self) {
        self.buffer.pop();
    }
}