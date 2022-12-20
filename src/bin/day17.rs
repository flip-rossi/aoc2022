//! Day 17: Pyroclastic Flow

use std::collections::{HashSet, HashMap};

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
fn part2(jets: Vec<i32>) -> ! {
    let mut map = HashSet::new();
    let mut max_height = 0;

    // Key: (jet, shape) -- Value: Vec<(drop, max_height)>
    let mut states: HashMap<(usize, usize), Vec<(usize, i32)>> = HashMap::new();

    let (delta_drops, delta_height);

    let mut drop = 0;
    'outer: loop {
        // eprintln!("{drop}");
        let jet = drop % jets.len();
        let shape = drop % SHAPES.len();

        let new_rock_pos = Pos::new(2, max_height+4);
        let mut rock = Rock::new(&SHAPES[shape], new_rock_pos);

        let mut rock_top = None;
        while let None = rock_top {
            rock.push_sideways(jets[jet], &map);
            rock_top = rock.fall_once(&mut map);
        }
        max_height = max_height.max(rock_top.unwrap());

        // cycle happened if for a same height interval, with the same jet and shape, the rock formation is the same
        // Final result is:
        //     detection_height
        //   + delta_height * drops_left / delta_drops
        //   + height added by the next `drops_left % delta_drops` drops
        match states.get_mut(&(jet, shape)) {
            None => { states.insert((jet, shape), vec![(drop, max_height)]); },
            Some(states) => {
                states.push((drop, max_height));
                for i in (0..states.len()-1).rev() {
                    let height_diff = max_height - states[i].1;
                    for j in (0..i).rev() {
                        if states[i].1 - states[j].1 == height_diff
                           && check_cycle(&map, max_height, height_diff) {
                            delta_drops = drop - states[i].0;
                            delta_height = height_diff;
                            break 'outer
                        }
                    }
                }

            },
        };

        drop += 1
    }

    eprintln!("Drop: {drop};\nMax height {max_height};\n(delta_drops, delta_height): ({delta_drops}, {delta_height}).");

    todo!()
}

fn check_cycle(map: &HashSet<Pos>, max_height: i32, height_diff: i32) -> bool {
    for y in (max_height - height_diff + 1)..=max_height {
        for x in 0..CHAMBER_WIDTH {
            // if x > 2 { panic!() }
            if map.contains(&Pos{x,y}) != map.contains(&Pos{x, y: y - height_diff}) {
                // eprintln!("diff at x={x}, y={y}");
                return false
            }
        }
    }
    true
}

