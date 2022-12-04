use std::io::{self, stdin};

macro_rules! pow {
    ($a:expr, $b: expr) => {{
        let mut ans = 1;
        for _ in 0..$b {
            ans *= $a;
        }
        ans
    }};
}

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

    pub fn as_numbers(&self) -> Vec<u32> {
        let mut numbers = Vec::new();

        let chars: Vec<char> = self.line.chars().collect();
        for i in (0..chars.len()).rev() {
            let mut num: Option<u32> = None;
            let mut order = 0;

            while let Some(d) = chars[i].to_digit(10) {
                num = match num {
                    Some(n) => Some(n + d*pow!(10,order)),
                    None => Some(d),
                };
                order += 1;
            }

            if let Some(n) = num {
                numbers.insert(0, n);
            }
        }

        numbers
    }
}

