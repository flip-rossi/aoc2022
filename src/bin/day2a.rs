extern crate aoc22;
use aoc22::line_reader::LineReader;

enum Outcome {
    W, L, D
}

fn main () {
    let mut lr = LineReader::new();

    let mut score = 0;
    while lr.read_next().unwrap() > 0 {
        let words = lr.as_words();

        let outcome = match words[1].as_str() {
            "X" => { //Rock
                score += 1;
                match words[0].as_str() {
                    "B" => Outcome::L,
                    "C" => Outcome::W,
                    _ => Outcome::D,
                }
            },
            "Y" => { //Paper
                score += 2;
                match words[0].as_str() {
                    "A" => Outcome::W,
                    "C" => Outcome::L,
                    _ => Outcome::D,
                }
            },
            "Z" => { //Scissor
                score += 3;
                match words[0].as_str() {
                    "A" => Outcome::L,
                    "B" => Outcome::W,
                    _ => Outcome::D,
                }
            },
            _ => panic!(),
        };

        match outcome {
            Outcome::L => score += 0,
            Outcome::D => score += 3,
            Outcome::W => score += 6,
        };
    }

    println!("Answer: {}", score)
}
