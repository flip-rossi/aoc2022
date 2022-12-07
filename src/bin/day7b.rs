//! Day 7 part 2 - No Space Left On Device

use aoc22::line_reader::LineReader;

const DRIVE_SIZE: isize = 70000000;
const UPDATE_SIZE: isize = 30000000;

fn main() {
    let mut lr = LineReader::new();

    let mut candidates: Vec<isize> = Vec::new();
    let mut wd: Vec<isize> = Vec::new();

    while lr.read_next().unwrap() > 0 {
        let mut words = lr.as_words().into_iter();

        match words.next().unwrap().as_str() {
            // Read command
            "$" => match words.next().unwrap().as_str() {
                    // cd command
                    "cd" => {
                        match words.next().unwrap().as_str() {
                            ".." => {
                                let last_wd = wd.pop().unwrap();
                                if let Some(dir) = wd.last_mut() {
                                    *dir += last_wd;
                                    candidates.push(last_wd);
                                }
                            },
                            "/" => {
                                wd = Vec::new();
                                wd.push(0);
                            },
                            _ => {
                                wd.push(0);
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
                    },
                    _ => panic!("Error in parsing command arguments.")
                },
            _ => panic!("Unexpected non-command line.")
        };
    }

    // Pop remaining dirs
    let mut missing_space = 0;
    while let Some(last_wd) = wd.pop() {
        if let Some(dir) = wd.last_mut() {
            *dir += last_wd;
        }
        else { //found root
            missing_space = UPDATE_SIZE - (DRIVE_SIZE - last_wd);
        }
        candidates.push(last_wd);
    }

    let mut answer = None;
    for size in candidates {
        match answer {
            None => if size >= missing_space { answer = Some(size) },
            Some(n) => if size >= missing_space && size < n {
                answer = Some(size);
            }
        }
    }

    match answer {
        Some(n) => println!("{n}"),
        None => eprintln!("Answer not found.")
    }
}

