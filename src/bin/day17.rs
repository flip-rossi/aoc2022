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
fn part1(jets: Vec<i32>) -> i64 {
    let mut map = HashSet::with_capacity(2022*5);
    let mut max_height = 0;

    let mut j = 0;
    for i in 0..ITERATIONS /*2022*/ {

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
        if i == 85 {
            println!("At iteration {i}: {max_height}");
        }
        else if i == ITERATIONS - 35 {
            println!("At iteration {i}: {max_height}");
        }
    }
    
    max_height as i64
}

const ITERATIONS: usize = 2022; // TODO remove this constant

//=============== PART 2 ===============//
fn part2(jets: Vec<i32>) -> i64 {
    let mut map = HashSet::new();
    let mut max_height = 0;
    //   heights[j] = Vec<(i, shape_index, height)>
    let mut jets_memo: Vec<Vec<(usize, usize, i32)>> = vec![Vec::new(); jets.len()];
    // (iteration_diff, height_diff)
    let saved_state;

    let mut i = 0;
    let mut j = 0;
    'outer: loop {
        let new_rock_pos = Pos::new(2, max_height+4);
        let shape_index = i % SHAPES.len();
        let mut rock = Rock::new(&SHAPES[shape_index], new_rock_pos);

        // print_map(&map, &rock);

        let mut rock_top = None;
        while let None = rock_top {
            rock.push_sideways(jets[j], &map);
            j = (j+1) % jets.len();
            rock_top = rock.fall_once(&mut map);
        }

        max_height = max_height.max(rock_top.unwrap());

        let new_memo = (i, shape_index, max_height);

        i+=1; //should this be after creating the new memo?

        for p in (0..jets_memo[j].len()).rev() {
            let prev_ind = p as isize - (jets_memo[j].len() - p) as isize;
            if prev_ind < 0 { continue }
            let prev_ind = prev_ind as usize;
            let height_diff = max_height - jets_memo[j][p].2;
            if jets_memo[j][p].2 - jets_memo[j][prev_ind].2 == height_diff &&
                jets_memo[j][p].1 == shape_index && jets_memo[j][prev_ind].1 == shape_index {
                eprintln!("PATTERN FOUND with:\n  jet {j};\n  shape {shape_index};\n  height_diff {height_diff};\n  height {max_height};\n  iteration {i}.");
                if check_formation(&map, max_height, height_diff) {
                    eprintln!("FORMATION FOUND with:\n  jet {j};\n  shape {shape_index};\n  height_diff {height_diff};\n  height {max_height};\n  iteration {i}\n  iter_diff: {}.",i - jets_memo[j][p].0);
                    saved_state = (i - jets_memo[j][p].0, height_diff);
                    break 'outer
                }
            }
        }
        jets_memo[j].push(new_memo);
    }


    let mut iters_left = ITERATIONS /* 1000000000000 */ - i;
    let maxer_height = max_height as i64 + (saved_state.1 as i64 * (iters_left / saved_state.0) as i64);
    iters_left = iters_left % saved_state.0;
    eprintln!("maxer_height: {maxer_height}\niters_left: {iters_left}");

    let old_max_height = max_height;
    let old_i = i;

    while i < iters_left + old_i {
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

        i+=1
    }

    eprintln!("Answer = {max_height} - {old_max_height} + maxer_height = {} + maxer_height", max_height - old_max_height);
    (max_height - old_max_height) as i64 + maxer_height
}

fn check_formation(map: &HashSet<Pos>, end: i32, height_diff: i32) -> bool {
    let start2 = end - height_diff;
    let start1 = end - height_diff - height_diff;
    for y in start1..start2 {
        for x in 0..CHAMBER_WIDTH {
            if map.contains(&Pos{x,y}) != map.contains(&Pos{x, y: y + height_diff}) {
                return false
            }
        }
    }

    true
}

