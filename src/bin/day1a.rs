extern crate aoc22;
use aoc22::line_reader::LineReader;

fn main() {
    let mut max = 0;
    let mut curr = 0;

    let mut line_r = LineReader::new();
    while line_r.read_next().unwrap() > 0 {
        curr = match isize::from_str_radix(&line_r.line, 10) {
            Ok(n) => curr + n,
            Err(_) => {
                if curr > max { max = curr }
                0
            },
        }
    }

    println!("Answer: {}", max)
}

