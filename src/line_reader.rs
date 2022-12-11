use std::io::{self, stdin};

pub struct LineReader {
    pub line: String, //Last line read as String
    pub count: usize, //Number of lines read
    pub stunned: bool,
}

impl LineReader {
    pub fn new() -> LineReader {
        LineReader {
            line: String::new(),
            count: 0,
            stunned: false,
        }
    }

    /**
    Reads the next line from stdin, saving it to self's `line` field.  
    Returns the amount of bytes read, or 0 for EOF.
    */
    pub fn read_next(&mut self) -> io::Result<usize> {
        if self.stunned {
            self.stunned = false;
            return Ok( self.line.len() )
        }

        self.line.clear();
        let nbytes = stdin().read_line(&mut self.line)?;
        if nbytes > 0 {
            self.line = self.line[..self.line.len()-1].into();
        }
        
        self.count += 1;
        Ok(nbytes)
    }

    /**
    Parses the `line` value as a Vec of words, where a word is
    any sequence of characters separated by spaces.
    */
    pub fn as_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        let mut w = String::new();

        for ch in self.line.chars() {
            if ch == ' ' {
                if !w.is_empty() {
                    words.push(w);
                    w = String::new();
                }
            } else {
                w.push(ch);
            }
        }

        if !w.is_empty() {
            words.push(w);
        }

        words
    }

    /**
    Parses the `line` value as a Vec of unsigned numbers,
    ignoring any character that's not a digit
    */
    pub fn as_numbers(&self, radix: usize) -> Vec<usize> {
        let mut numbers = Vec::new();

        let chars: Vec<char> = self.line.chars().collect();
        let mut i = chars.len();
        while i > 0 {
            i -= 1;
            
            let mut num: Option<usize> = None;
            let mut order = 0;

            while let Some(d) = chars[i].to_digit(10) {
                num = match num {
                    Some(n) => Some( n + (d as usize)*radix.pow(order) ),
                    None => Some(d as usize),
                };
                order += 1;
                if i == 0 { break; }
                i -= 1;
            }

            if let Some(n) = num {
                numbers.insert(0, n);
            }
        }

        numbers
    }
}

