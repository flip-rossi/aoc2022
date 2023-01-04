//! Day 21: Monkey Math

use std::collections::HashMap;

#[derive(Debug)]
enum Monkey {
    Num(i64),
    Op(String, String, String)
}
impl Monkey {
    fn yell(&self, other_monkeys: &HashMap<String,Monkey>) -> i64 {
        match self {
            Monkey::Num(num) => *num,
            Monkey::Op(monkey1, op, monkey2) => {
                let num1 = other_monkeys.get(monkey1).unwrap().yell(other_monkeys);
                let num2 = other_monkeys.get(monkey2).unwrap().yell(other_monkeys);
                match op.as_str() {
                    "+" => num1 + num2,
                    "-" => num1 - num2,
                    "*" => num1 * num2,
                    "/" => num1 / num2,
                    _ => panic!("monkey yell: bad operator")
                }
            }
        }
    }
}

fn main() {
    let mut monkeys: HashMap<String,Monkey> = HashMap::new();
    // Parse input
    for line in std::io::stdin().lines().map(|s| s.unwrap().trim().to_string()) {
        let mut words = line.split(&[':', ' ']).filter(|s| !s.is_empty());

        let name = words.next().unwrap();
        let arg1 = words.next().unwrap();
        let monkey = match arg1.parse::<i64>() {
            Ok(num) => Monkey::Num(num),
            Err(_) => {
                let arg2 = words.next().unwrap();
                let arg3 = words.next().unwrap();
                Monkey::Op(String::from(arg1), String::from(arg2), String::from(arg3))
            },
        };
        monkeys.insert(String::from(name), monkey);
    }

    // Solve
    let answer = aoc22::solve_puzzle!(monkeys);
    println!("{answer}")
}

//=============== PART 1 ===============//
fn part1(monkeys: HashMap<String,Monkey>) -> i64 {
    monkeys.get("root").unwrap().yell(&monkeys)
}

//=============== PART 2 ===============//
#[allow(dead_code, unused_variables)]
fn part2(monkeys: HashMap<String,Monkey>) -> ! {
    todo!()
}

