extern crate aoc22;
use aoc22::line_reader::LineReader;

enum Choice {
    R, P, S
}

fn main () {
    let mut lr = LineReader::new();

    let mut score = 0;
    while lr.read_next().unwrap() > 0 {
        let words = lr.as_words();

        let outcome = match words[1].as_str() {
            "X" => { //Lose
                score += 0;
                match words[0].as_str() {
                    "B" => Choice::R,
                    "C" => Choice::P,
                    _ => Choice::S,
                }
            },
            "Y" => { //Draw
                score += 3;
                match words[0].as_str() {
                    "A" => Choice::R,
                    "C" => Choice::S,
                    _ => Choice::P,
                }
            },
            "Z" => { //Win
                score += 6;
                match words[0].as_str() {
                    "A" => Choice::P,
                    "B" => Choice::S,
                    _ => Choice::R,
                }
            },
            _ => panic!(),
        };

        match outcome {
            Choice::R => score += 1,
            Choice::P => score += 2,
            Choice::S => score += 3,
        };
    }

    println!("Answer: {}", score)
}
