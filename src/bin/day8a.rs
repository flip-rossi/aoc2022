//! Day 8 part 1 - Treetop Tree House

use aoc22::line_reader::LineReader;

fn main() {
    // Fill grid
    let mut grid: Vec<Vec<(u32, bool)>> = Vec::new(); // the inner tuple is (tree_size, is_visible)
    let mut lr = LineReader::new();
    while lr.read_next().unwrap() > 0 {
        let digits = lr.line.as_str().chars()
            .map(|c| (c.to_digit(10).expect("Non-digit character {c} found."), false))
            .collect();
        grid.push(digits);
    }

    // Calculate answer
    let mut answer = grid.len()*2 + grid[0].len()*2 - 4;
    // horizontally
    for i in 1..grid.len()-1 {
        // left to right
        let mut tallest = grid[i].first().unwrap().0;
        for j in 1..grid[i].len()-1 {
            if tree_becomes_visible(&mut grid[i][j], &mut tallest) {
                answer += 1;
            }
        }
        // right to left
        tallest = grid[i].last().unwrap().0;
        for j in ( 1..grid[i].len()-1 ).rev() {
            if tree_becomes_visible(&mut grid[i][j], &mut tallest) {
                answer += 1;
            }
        }
    }
    // vertically
    for j in 1..grid[0].len()-1 {
        // top to bottom
        let mut tallest = grid.first().unwrap()[j].0;
        for i in 1..grid.len()-1 {
            if tree_becomes_visible(&mut grid[i][j], &mut tallest) {
                answer += 1;
            }
        }
        // right to left
        tallest = grid.last().unwrap()[j].0;
        for i in ( 1..grid.len()-1 ).rev() {
            if tree_becomes_visible(&mut grid[i][j], &mut tallest) {
                answer += 1;
            }
        }
    }

    println!("{answer}");
}

fn tree_becomes_visible(tree: &mut (u32,bool), tallest: &mut u32) -> bool {
    if tree.0 > *tallest {
        *tallest = tree.0;
        if !tree.1 {
            tree.1 = true;
            return true
        }
    }
    false
}

