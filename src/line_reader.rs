use std::io::{self, stdin};

pub struct LineReader {
    pub line: String, //Last line read as String
    pub count: usize, //Number of lines read
}

impl LineReader {
    pub fn new() -> LineReader {
        LineReader {
            line: String::new(),
            count: 0,
        }
    }

    pub fn read_next(&mut self) -> io::Result<usize> {
        self.line.clear();
        let nbytes = stdin().read_line(&mut self.line)?;
        if nbytes > 0 {
            self.line = self.line[..self.line.len()-1].into();
        }
        
        self.count += 1;
        Ok(nbytes)
    }

    pub fn as_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        let mut w = String::new();

        // NOTE: doesn't account for repeated spaces.
        for ch in self.line.chars() {
            if ch == ' ' {
                words.push(w);
                w = String::new();
            } else {
                w.push(ch);
            }
        }
        words.push(w);

        words
    }
}
