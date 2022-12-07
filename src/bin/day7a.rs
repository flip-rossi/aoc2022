//! Day 7 part 1 - No Space Left On Device

use aoc22::line_reader::LineReader;

const MAX_DIR_SIZE: isize = 100000;

fn main() {
    let mut lr = LineReader::new();
    let mut answer = 0;

    let mut wd: Vec<isize> = Vec::new();
    while lr.read_next().unwrap() > 0 {
        let mut words = lr.as_words().into_iter();

        let last_size = match words.next().unwrap().as_str() {
            // Read command
            "$" => match words.next().unwrap().as_str() {
                    // cd command
                    "cd" => {
                        match words.next().unwrap().as_str() {
                            ".." => {
                                let last_wd = wd.pop().unwrap();
                                if let Some(dir) = wd.last_mut() {
                                    *dir += last_wd;
                                }
                                Some(last_wd)
                            },
                            "/" => {
                                wd = Vec::new();
                                wd.push(0);
                                None
                            },
                            _ => {
                                wd.push(0);
                                None
                            }
                        }
                    },
                    // ls command
                    "ls" => {
                        while lr.read_next().unwrap() > 0 {
                            let mut words = lr.as_words().into_iter();
                            match words.next().unwrap().as_str() {
                                "dir" => { },
                                "$" => {
                                    lr.stunned = true;
                                    break;
                                },
                                size_str => {
                                    match isize::from_str_radix(size_str, 10) {
                                        Ok(size) => {
                                            match words.next() {
                                                Some(_) => *wd.last_mut().unwrap() += size,
                                                None => panic!("ls: expected file name")
                                            }
                                        }
                                        _ => panic!("ls: unexpected output.")
                                    }
                                }
                            }
                        }
                        None
                    },
                    _ => panic!("Error in parsing command arguments.")
                },
            _ => panic!("Unexpected non-command line.")
        };

        if let Some(n) = last_size {
            if n <= MAX_DIR_SIZE {
                answer += n;
            }
        }
    }

    println!("{answer}")
}

