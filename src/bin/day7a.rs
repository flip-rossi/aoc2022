//! Day 7 part 1 - No Space Left On Device

use aoc22::line_reader::LineReader;

const MAX_DIR_SIZE: isize = 100000;

struct Directory {
    name: String,//Box<String>,
    files: Vec<File>,
    sub_dirs: Vec<Directory>,
    total_size: isize,
}
impl Directory {
    fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            files: Vec::new(),
            sub_dirs: Vec::new(),
            total_size: 0,
        }
    }

    fn add_file(&mut self, file: File) {
        self.total_size += file.size;
        self.files.push(file);
    }
}

struct File {
    name: String,
    size: isize,
}
impl File{
    fn new<S: Into<String>>(name: S, size: isize) -> Self {
        Self { name: name.into(), size, }
    }
}

fn main() {
    let mut lr = LineReader::new();
    let mut answer = 0;

    let mut wd: Vec<Directory> = Vec::new();
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
                                    dir.add_file(File::new(last_wd.name, last_wd.total_size));
                                }
                                Some(last_wd.total_size)
                            },
                            "/" => {
                                wd = Vec::new();
                                wd.push(Directory::new("/"));
                                None
                            },
                            dir_name => {
                                wd.push(Directory::new(dir_name));
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
                                                Some(file_name) => wd.last_mut().unwrap().add_file(File::new(file_name, size)),
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

