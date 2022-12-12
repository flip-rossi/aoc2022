//! Day 12: Hill Climbing Algorithm

use aoc22::{solve_puzzle, position::Pos};

const DIRECTIONS: [Pos; 4] = [
    Pos{x: 0,y:-1}, // Up
    Pos{x: 0,y: 1}, // Down
    Pos{x:-1,y: 0}, // Left
    Pos{x: 1,y: 0}, // Right
];

fn main() {
    // Parse input
    let mut terrain = Vec::new(); // each item in the matrix is (height, visited)
    let mut start = Pos::new(0,0);
    let mut end = Pos::new(0,0);

    let lines = std::io::stdin().lines();
    let mut i = 0;
    for l in lines {
        let line = l.unwrap();
        let mut row = Vec::with_capacity(line.len());

        let bytes = line.bytes();
        let mut j = 0;
        for b in bytes {
            row.push(match b {
                0x53 => { start = Pos::new(i,j); (0, Tag::Perm(0)) }, // 'S' in ascii. The start is 'a'
                0x45 => { end = Pos::new(i,j); (25, Tag::Empty) }, // 'E' in ascii. The end is 'z'
                _ => (b - 0x61, Tag::Empty),
            });
            j += 1
        }
        terrain.push(row);
        i += 1
    }

    // Solve
    let answer = solve_puzzle!(terrain, start, end);
    println!("{answer}")
}

//=============== PART 1 ===============//
#[derive(Clone, Copy, Debug)]
enum Tag {
    Temp(i32),
    Perm(i32),
    Empty,
}
impl Tag {
    fn update(&mut self, new_val: i32) -> i32 {
        let mut end_val = new_val;
        *self = match self {
            Tag::Empty => Tag::Temp(new_val),
            Tag::Temp(old_val) => if new_val < *old_val {
                    Tag::Temp(new_val)
                }
                else {
                    end_val = *old_val;
                    *self
                },
            Tag::Perm(old_val) => { end_val = *old_val; *self },
        };
        end_val
    }

    fn make_perm(&mut self) {
        if let Tag::Temp(val) = self {
            *self = Tag::Perm(*val)
        }
    }
}

fn dijkstras(mut terrain: Vec<Vec<(u8,Tag)>>, start: Pos, end: Pos) -> i32 {
    // let mut nodes: Vec<Vec<(u8,Tag)>> = terrain.iter()
    //     .map(|i| i.iter()
    //         .map(|j| (*j, Tag::Empty)).collect())
    //     .collect();

    let mut last_permmed: Vec<(Pos,Tag)> = Vec::new();
    let ustart = start.as_usize_tuple().unwrap();
    last_permmed.push((start, terrain[ustart.0][ustart.1].1));
    let mut tmp_tags: std::collections::HashMap<Pos, Tag> = std::collections::HashMap::new();

    let mut iterations = 0;

    let uend = end.as_usize_tuple().unwrap();
    while let Tag::Empty|Tag::Temp(_) = terrain[uend.0][uend.1].1 {
        eprintln!("ITERATION {iterations}");
        eprintln!("Edge tags: {}", last_permmed.len());

        // Find new minimal tags
        for tag in &last_permmed {
            let tag_val = match tag.1 {
                Tag::Perm(v) => v,
                _ => panic!("Non-permanent tag in last_permmed list")
            };
            for dir in DIRECTIONS {
                let next_pos = tag.0 + dir;
                if is_valid_movement(tag.0, next_pos, &terrain) {
                    let unext = next_pos.as_usize_tuple().unwrap();
                    terrain[unext.0][unext.1].1.update(tag_val + 1);
                    tmp_tags.insert(next_pos, terrain[unext.0][unext.1].1);
                    eprintln!("{tag_val}");
                }
            }
        }
        last_permmed.clear();

        // Find min-val
        let mut min_val = i32::MAX;
        for tag in &tmp_tags {
            if let Tag::Temp(val) = tag.1 {
                if val < &min_val {
                    min_val = *val;
                }
            }
        }

        // Make new minimal tags be permanent
        let mut tags_made_perm = Vec::new();
        for tag in &tmp_tags {
            if let Tag::Temp(val) = tag.1 {
                if val <= &min_val {
                    let upos = tag.0.as_usize_tuple().unwrap();
                    terrain[upos.0][upos.1].1.make_perm();
                    tags_made_perm.push(*tag.0);
                    last_permmed.push((*tag.0, terrain[upos.0][upos.1].1));
                }
            }
        }
        for pos in &tags_made_perm {
            tmp_tags.remove(pos);
        }

        iterations += 1
    }

    if let Tag::Perm(shortest) = terrain[uend.0][uend.1].1 {
        shortest
    } else {
        panic!("Dijkstra's: Couldn't find a path.")
    }
}

fn is_valid_movement(pos: Pos, next_pos: Pos, terrain: &Vec<Vec<(u8,Tag)>>) -> bool {
    if next_pos.x < 0 || next_pos.y < 0 {
        return false
    }

    let upos = pos.as_usize_tuple().unwrap();
    let unext = next_pos.as_usize_tuple().unwrap();

    if unext.0 >= terrain.len() || unext.1 >= terrain[unext.0].len() {
        return false
    }

    let curr_square = &terrain[upos.0][upos.1];
    let next_square = &terrain[unext.0][unext.1];

    eprintln!("[{}][{}] {:?}", unext.0, unext.1, next_square.1);
    if let Tag::Perm(_) = next_square.1 {
        return false
    }

    return next_square.0 <= curr_square.0 + 1
}

fn part1(terrain: Vec<Vec<(u8,Tag)>>, start: Pos, end: Pos) -> i32 {
    dijkstras(terrain, start, end)
}

//=============== PART 2 ===============//
#[allow(unused_variables)]
fn part2(terrain: Vec<Vec<(u8,Tag)>>, start: Pos, end: Pos) -> ! {
    todo!()
}

