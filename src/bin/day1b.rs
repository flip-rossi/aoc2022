//! Day 1 part 2 - Calorie Counting

use aoc22::line_reader::LineReader;

fn main() {
    let mut maxes = [0,0,0]; //sorted highest to lowest
    let mut curr = 0;

    let mut line_r = LineReader::new();
    while line_r.read_next().unwrap() > 0 {
        curr = match isize::from_str_radix(&line_r.line, 10) {
            Ok(n) => curr + n,
            Err(_) => {
                for i in 0..3 {
                    if curr > maxes[i] {
                        for j in (i..2).rev() {
                            maxes[j+1] = maxes[j];
                        }
                        maxes[i] = curr;
                        break;
                    }
                }
                0
            },
        }
    }

    let mut sum = 0;
    for m in maxes {
        sum += m;
    }
    println!("Answer: Sum of {:?} = {}", maxes, sum)
}

