pub struct Document {
    pub lines: Vec<String>,
    pub path: String,
}

impl Document {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            path: "Untitled.txt".to_string(),
        }
    }

    pub fn insert_char(&mut self, x: usize, y: usize, c: char) {
        self.lines[y].insert(x, c);
    }

    pub fn delete_char(&mut self, x: usize, y: usize) {
        self.lines[y].remove(x);
    }

    pub fn insert_newline(&mut self, x: usize, y: usize) {
        let remainder = self.lines[y].split_off(x);
        self.lines.insert(y+1, remainder);
    }

    pub fn join_lines(&mut self, y: usize) {
        if y <= self.lines.len() {
            let next = self.lines.remove(y+1);
            self.lines[y].push_str(&next);
        }
    }

    pub fn open_file(&mut self, path: &str) {
        if let Ok(contents) = std::fs::read_to_string(path) {
            self.lines = contents.lines().map(String::from).collect();
            self.path = path.to_string();
        }
    }

    pub fn save_file(&self, path: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path).expect("Unable to create file");
        for line in &self.lines {
            writeln!(file, "{}", line).expect("Unable to write to file");
        }

        Ok(())
    }
}