//! Day 11: Monkey in the Middle

use aoc22::line_reader::LineReader;

#[derive(Debug)]
enum Operation {
    Plus(i64),
    Times(i64),
    TimesOld,
    None
}
impl Operation {
    fn set_value(&mut self, x: i64) {
        match self {
            Self::Plus(n)|Self::Times(n) => *n = x,
            _ => {},
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test_value: i64,
    targets: (usize, usize),
    inspections: i32,
}
impl Monkey {
    fn new() -> Self {
        Self { items: Vec::new(), operation: Operation::None, test_value: 0, targets: (0,0), inspections: 0 }
    }
}

fn main() {
    // Parse input
    let mut lr = LineReader::new();
    let mut monkeys = Vec::new();
    let mut monkey = Monkey::new();
    while lr.read_next().unwrap() > 0 {
        let words = lr.as_words();
        match words.first().as_deref().map(|s| &s[..]) {
            Some("Monkey") => {},
            Some("Starting") => monkey.items = lr.as_numbers(10).iter().map(|n| *n as i64).collect(),
            Some("Operation:") => {
                let mut operation = Operation::None;
                for w in words {
                    match w.as_str() {
                        "*" => operation = Operation::Times(0),
                        "+" => operation = Operation::Plus(0),
                        "old" => if let Operation::Times(_) = operation { operation = Operation::TimesOld }
                        s => match i64::from_str_radix(s, 10) {
                            Ok(n) => operation.set_value(n),
                            Err(_) => {},
                        }
                    }
                }
                monkey.operation = operation;
            },
            Some("Test:") => {
                monkey.test_value = *lr.as_numbers(10).first().unwrap() as i64;
                lr.read_next().unwrap();
                monkey.targets.0 = *lr.as_numbers(10).first().unwrap();
                lr.read_next().unwrap();
                monkey.targets.1 = *lr.as_numbers(10).first().unwrap();
            },
            None|Some(_) => {
                eprintln!("Monkey {}: {monkey:?}", monkeys.len());
                monkeys.push(monkey);
                monkey = Monkey::new();
            },
        }
    }
    eprintln!("Monkey {}: {monkey:?}", monkeys.len());
    monkeys.push(monkey); // push last monkey

    // Solve
    let answer = part1(monkeys);
    println!("{answer}")
}

//=============== PART 1 ===============//
const ROUNDS: i32 = 20;

fn part1(mut monkeys: Vec<Monkey>) -> i32 {
    // Do the monkey business
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            m.inspections += m.items.len() as i32;

            let mut throws: Vec<(i64, usize)> = Vec::with_capacity(m.items.len());
            for _ in 0..m.items.len() {

                let mut item = m.items.pop().unwrap();
                match m.operation {
                    Operation::None => panic!("Invalid operation for monkey {i}: {m:?}"),
                    Operation::Plus(n) => item = (item + n)/3,
                    Operation::Times(n) => item = (item * n)/3,
                    Operation::TimesOld => item = (item * item)/3,
                }

                if item % m.test_value == 0 {
                    throws.push((item, m.targets.0));
                }
                else {
                    throws.push((item, m.targets.1));
                }
            }

            for t in throws {
                monkeys[t.1].items.push(t.0);
            }
        }
    }

    // Get answer
    let mut top2 = [0; 2];
    for m in monkeys {
        if m.inspections > top2[1] {
            if m.inspections > top2[0] {
                top2[1] = top2[0];
                top2[0] = m.inspections;
            }
            else {
                top2[1] = m.inspections;
            }
        }
    }

    top2[0] * top2[1]
}

