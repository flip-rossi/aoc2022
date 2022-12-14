//! Day 13: Distress Signal

use std::cmp::Ordering::{Greater, Equal, Less};

use aoc22::solve_puzzle;

#[derive(Clone, Eq, Ord)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}
impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => f.write_fmt(format_args!("{num}")),
            Self::List(list) => f.write_fmt(format_args!("{list:?}")),
        }
    }
}
impl Item {
    fn push_subitem(&mut self, subitem: Item) -> Result<(),()> {
        match self {
            Item::List(list) => { list.push(subitem); Ok(()) }
            _ => Err(())
        }
    }

    fn num_as_list(&self) -> Result<Item,()> {
        match self {
            Item::Number(num) => Ok( Item::List(vec![Item::Number(*num)]) ),
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
                Self::List(o_list) => {
                    if o_list.len() == 0 { Some(Greater) }
                    else { self.num_as_list().unwrap().partial_cmp(&other) }
                },
            },
            Self::List(s_list) => match other {
                Self::Number(_o_num) => {
                    if s_list.len() == 0 { Some(Less) }
                    else { self.partial_cmp(&other.num_as_list().unwrap()) }
                },
                Self::List(o_list) => {
                    let min_length = s_list.len().min(o_list.len());
                    for i in 0..min_length {
                        let ordering = s_list[i].partial_cmp(&o_list[i]);
                        if let Some(Less|Greater) = ordering
                            { return ordering }
                    }
                    s_list.len().partial_cmp(&o_list.len())
                },
            },
        }
    }
}

fn main() {
    // Parse input
    let mut packets: Vec<Item> = Vec::new();

    let stdin = std::io::stdin();
    let mut line = String::new();
    while stdin.read_line(&mut line).unwrap() > 0 {
        for _ in 0..2 {
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
            if !num_item.is_empty() {
                push_num_item(&mut list_item, &mut num_item);
            }

            eprintln!("{:?}", list_item);
            packets.push(list_item);
            line.clear();
            stdin.read_line(&mut line).unwrap();
        }
        line.clear();
        eprintln!();
    }

    // Solve
    let answer = solve_puzzle!(packets);
    println!("{answer}")
}


//=============== PART 1 ===============//
fn part1(packets: Vec<Item>) -> usize {
    eprintln!("=============");
    let mut sum = 0;
    for i in 0..packets.len()/2 {
        if packets[i*2] < packets[i*2+1] {
            eprintln!("PAIR {}\n{:?}\nis smaller than\n{:?}", i+1, packets[i*2], packets[i*2+1]);
            sum += i+1;
        }
    }
    sum
}

//=============== PART 2 ===============//
fn part2(packets: Vec<Item>) -> usize {
    let div_packates = (
        Item::List(vec![Item::List(vec![Item::Number(2)])]),
        Item::List(vec![Item::List(vec![Item::Number(6)])])
    );

    let mut indexes = (1,1);
    let mut greater_packates = Vec::with_capacity(packets.len());
    for p in packets {
        if div_packates.0 > p {
            indexes.0 += 1;
        }
        else {
            greater_packates.push(p.clone());
        }
    }

    indexes.1 = indexes.0 + 1;
    for p in greater_packates {
        if div_packates.1 > p {
            indexes.1 += 1;
        }
    }

    indexes.0 * indexes.1
}

