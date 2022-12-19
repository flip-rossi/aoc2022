//! Day 17: Pyroclastic Flow

use std::collections::HashSet;

use aoc22::{solve_puzzle, position::Pos};

fn main() {
    // Parse Input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let jets = input.trim().chars()
        .map(|ch| match ch {
            '<' => -1,
            '>' =>  1,
            ch => panic!("bad input: {ch}")
        })
        .collect::<Vec<i32>>();

    // Solve
    let answer = solve_puzzle!(jets);
    println!("{answer}")
}

const CHAMBER_WIDTH: i32 = 7;
const SHAPES: [[(i32,i32);5];5] = [
    [(0,0),(1,0),(2,0),(3,0)      ,(-1,-1)],
    [(1,0),(0,1),(1,1),(2,1),(1,2)],
    [(0,0),(1,0),(2,0),(2,1),(2,2)],
    [(0,0),(0,1),(0,2),(0,3)      ,(-1,-1)],
    [(0,0),(1,0),(0,1),(1,1)      ,(-1,-1)],
];

struct Rock {
    pos: Pos,
    shape: Vec<Pos>
}
impl Rock {
    fn new(shape: &[(i32,i32)], pos: Pos) -> Self {
        let mut new_rock = Rock {
            pos,
            shape: Vec::with_capacity(5),
        };
        for p in shape {
            if *p == (-1,-1) {
                break
            }
            new_rock.shape.push(Pos::new(p.0, p.1))
        }
        new_rock
    }

    fn push_sideways(&mut self, side: i32, map: &HashSet<Pos>) -> bool {
        let dir = Pos::new(side, 0);
        for square in &self.shape {
            let next_pos = self.pos + *square + dir;
            if next_pos.x < 0 || next_pos.x >= CHAMBER_WIDTH || map.contains(&next_pos) {
                return false
            }
        }
        self.pos += dir;
        true
    }

    fn fall_once(&mut self, map: &mut HashSet<Pos>) -> Option<i32> {
        let dir = Pos::new(0, -1);
        for square in &self.shape {
            let next_pos = self.pos + *square + dir;
            if next_pos.y <= 0 || map.contains(&next_pos) {
                let mut max_height = 0;
                for square in &self.shape {
                    let height = self.pos + *square;
                    map.insert(height);
                    max_height = max_height.max(height.y);
                }
                return Some(max_height)
            }
        }
        self.pos += dir;
        None
    }
}

#[allow(dead_code)]
fn print_map(map: &HashSet<Pos>, rock: &Rock) {
    let mut rock_positions = HashSet::with_capacity(5);
    let mut rock_top = 0;
    for square in &rock.shape {
        rock_positions.insert(rock.pos + *square);
        rock_top = rock_top.max(rock.pos.y + square.y);
    }

    let mut outupt = String::new();
    for y in (1..rock_top+1).rev() {
        outupt.push('|');
        for x in 0..CHAMBER_WIDTH {
            outupt.push(
                if rock_positions.contains(&Pos{x,y}) {'@'}
                else if map.contains(&Pos{x,y})       {'#'}
                else {'.'}
            )
        }
        outupt.push_str("|\n");
    }
    outupt.push_str("+-------+\n");
    println!("{outupt}");
}

//=============== PART 1 ===============//
fn part1(jets: Vec<i32>) -> i32 {
    let mut map = HashSet::with_capacity(2022*5);
    let mut max_height = 0;

    let mut j = 0;
    for i in 0..2022 {
        let new_rock_pos = Pos::new(2, max_height+4);
        let mut rock = Rock::new(&SHAPES[i % SHAPES.len()], new_rock_pos);

        // print_map(&map, &rock);

        let mut rock_top = None;
        while let None = rock_top {
            rock.push_sideways(jets[j], &map);
            j = (j+1) % jets.len();
            rock_top = rock.fall_once(&mut map);
        }

        max_height = max_height.max(rock_top.unwrap());
    }
    
    max_height
}

//=============== PART 2 ===============//
#[allow(unused_variables)]
fn part2(jets: Vec<i32>) -> ! {
    todo!()
}

