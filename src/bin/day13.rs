//! Day 13: Distress Signal

use std::cmp::Ordering::{Greater, Equal, Less};

use aoc22::solve_puzzle;

#[derive(Debug, Eq, Ord)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}
impl Item {
    fn push_subitem(&mut self, subitem: Item) -> Result<(),()> {
        match self {
            Item::List(list) => { list.push(subitem); Ok(()) }
            _ => Err(())
        }
    }
    
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        let eq_list_and_num = |list: &Vec<Self>, num| list.len() == 1 && list[0].eq(num);
        match self {
            Self::Number(s_num) => match other {
                Self::Number(o_num) => s_num == o_num,
                Self::List(o_list) => eq_list_and_num(o_list,self),
            },
            Self::List(s_list) => match other {
                Self::Number(_o_num) => eq_list_and_num(s_list,other),
                Self::List(o_list) => {
                    for i in s_list {
                        if !i.eq(other) { return false }
                    }
                    if s_list.len() != o_list.len() { false }
                    else { true }
                }
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl PartialOrd for Item {
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other { return Some(Equal) }

        match self {
            Self::Number(s_num) => match other {
                Self::Number(o_num) => s_num.partial_cmp(o_num),
                Self::List(o_list) => Some( 
                    if o_list.len() == 0 { Greater }
                    else { Less } // because Equal was already checked
                ),
            },
            Self::List(s_list) => match other {
                Self::Number(_o_num) => Some(
                    if s_list.len() == 0 { Less }
                    else { Greater } // because Equal was already checked
                ),
                Self::List(o_list) => {
                    for i in s_list {
                        let ordering = i.partial_cmp(other);
                        if let Some(Less|Greater) = ordering { return ordering }
                    }
                    s_list.len().partial_cmp(&o_list.len())
                },
            },
        }
    }
}

fn main() {
    // Parse input
    let mut pairs: Vec<(Item,Item)> = Vec::new();

    let stdin = std::io::stdin();
    let mut line = String::new();
    while stdin.read_line(&mut line).unwrap() > 0 {
        let mut packets: [Option<Item>; 2] = [None, None];
        for i in 0..=1 {
            let mut item_stack = Vec::new();
            let mut list_item = Item::List(Vec::new());
            let mut num_item = String::new();

            let push_num_item = |list_item: &mut Item, num_item: &mut String| {
                let num = i32::from_str_radix(&num_item, 10).unwrap();
                num_item.clear();
                list_item.push_subitem(Item::Number(num)).unwrap();
            };

            for ch in line[1..line.len()-2].chars() {
                match ch {
                    '[' => {
                        item_stack.push(list_item);
                        list_item = Item::List(Vec::new());
                    },
                    ']' => {
                        if !num_item.is_empty() {
                            push_num_item(&mut list_item, &mut num_item);
                        }
                        let mut father = item_stack.pop().unwrap();
                        father.push_subitem(list_item).unwrap();
                        list_item = father;
                    }
                    ',' => {
                        if !num_item.is_empty() {
                            push_num_item(&mut list_item, &mut num_item);
                        }
                    }
                    ch => {
                        num_item.push(ch);
                    },
                }
            }

            eprintln!("{:?}", list_item);
            packets[i] = Some(list_item);
            line.clear();
            stdin.read_line(&mut line).unwrap();
        }
        line.clear();
        pairs.push( (packets[0].take().unwrap(), packets[1].take().unwrap()) );
    }

    // Solve
    let answer = solve_puzzle!();
    println!("{answer}")
}


//=============== PART 1 ===============//
fn part1() -> usize {
    todo!()
}

//=============== PART 2 ===============//
fn part2() -> ! {
    todo!()
}

