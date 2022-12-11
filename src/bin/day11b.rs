//! Day 11: Monkey in the Middle

use aoc22::line_reader::LineReader;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Plus(i32),
    Times(i32),
    TimesOld,
    None
}
impl Operation {
    fn set_value(&mut self, x: i32) {
        match self {
            Self::Plus(n)|Self::Times(n) => *n = x,
            _ => {},
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items_held: Vec<usize>, //index of the item held in the items values list
    items_values: Vec<i32>, //the values of the items related to the monkey's test value
    operation: Operation,
    test_value: i32,
    targets: (usize, usize),
    inspections: i64,
}
impl Monkey {
    fn new() -> Self {
        Self { 
            items_held: Vec::new(), items_values: Vec::new(),
            operation: Operation::None,
            test_value: 0, targets: (0,0),
            inspections: 0,
        }
    }

    fn update_item(&mut self, item_index: usize, oper: Operation) {
        let mut item = self.items_values[item_index];
        item = match oper {
            Operation::None => panic!("Operation `None` when trying to update item"),
            Operation::Plus(x) => item + x,
            Operation::Times(x) => item * x,
            Operation::TimesOld => item * item,
        } % self.test_value;
        self.items_values[item_index] = item;
    }
}

fn main() {
    // Parse input
    let mut lr = LineReader::new();
    let mut monkeys = Vec::new();
    let mut items = Vec::new();
    let mut monkey = Monkey::new();
    while lr.read_next().unwrap() > 0 {
        let words = lr.as_words();
        match words.first().as_deref().map(|s| &s[..]) {
            Some("Monkey") => {},
            Some("Starting") => { //monkey.items = lr.as_numbers(10).iter().map(|n| *n as i64).collect(),
                let monkey_items = lr.as_numbers(10);
                for i in monkey_items {
                    items.push(i as i32);
                    monkey.items_held.push(items.len()-1);
                }
            }
            Some("Operation:") => {
                let mut operation = Operation::None;
                for w in words {
                    match w.as_str() {
                        "*" => operation = Operation::Times(0),
                        "+" => operation = Operation::Plus(0),
                        "old" => if let Operation::Times(_) = operation { operation = Operation::TimesOld }
                        s => match i32::from_str_radix(s, 10) {
                            Ok(n) => operation.set_value(n),
                            Err(_) => {},
                        }
                    }
                }
                monkey.operation = operation;
            },
            Some("Test:") => {
                monkey.test_value = *lr.as_numbers(10).first().unwrap() as i32;
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

    // Fill every monkey's item values list
    for i in items {
        for m in &mut monkeys {
            let test_value = m.test_value;
            m.items_values.push(i % test_value);
        }
    }

    // Solve
    let answer = part2(monkeys);
    println!("{answer}")
}

//=============== PART 2 ===============//
const ROUNDS: i32 = 10_000;
fn part2(mut monkeys: Vec<Monkey>) -> i64 {
    // Do the monkey business
    for _ in 0..ROUNDS {
        for monkey_index in 0..monkeys.len() {
            // Update items
            let m = &monkeys[monkey_index];
            let items_held = m.items_held.clone();
            let operation = m.operation.clone();
            for item in &items_held {
                for mon in &mut monkeys {
                    mon.update_item(*item, operation);
                }
            }

            // Throw stuff
            let mut m = &mut monkeys[monkey_index];
            //         Vec<(item_index, monkey_index)>
            let mut throws: Vec<(usize, usize)> = Vec::with_capacity(m.items_held.len());
            for &item in &m.items_held {
                if m.items_values[item] == 0 {
                    throws.push((item, m.targets.0));
                }
                else {
                    throws.push((item, m.targets.1));
                }
            }
            m.inspections += m.items_held.len() as i64;
            m.items_held.clear();

            for t in throws {
                monkeys[t.1].items_held.push(t.0);
            }
        }
    }

    // Get answer
    let mut top2: [i64; 2] = [0; 2];
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

