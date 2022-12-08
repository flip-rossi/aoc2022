//! Day 8 part 1 - Treetop Tree House

use aoc22::line_reader::LineReader;

fn main() {
    // Fill grid
    let mut grid: Vec<Vec<u32>> = Vec::new(); 
    let mut lr = LineReader::new();
    while lr.read_next().unwrap() > 0 {
        let digits = lr.line.as_str().chars()
            .map(|c| c.to_digit(10).expect("Non-digit character {c} found."))
            .collect();
        grid.push(digits);
    }

    // Calculate answer
    let mut answer = 0;

    for i in 1..grid.len()-1 {
        for j in 1..grid[i].len()-1 {
            let this_tree = grid[i][j];
            let mut score = 1;
            let mut dir_score; let mut k;
            //look left
            dir_score = 0; k = j-1;
            while k > 0 && grid[i][k] < this_tree {
                dir_score += 1;
                k -= 1;
            }
            score *= dir_score+1;
            // look right
            dir_score = 0; k = j+1;
            while k < grid[i].len()-1 && grid[i][k] < this_tree {
                dir_score += 1;
                k += 1;
            }
            score *= dir_score+1;
            // look up
            dir_score = 0; k = i-1;
            while k > 0 && grid[k][j] < this_tree {
                dir_score += 1;
                k -= 1;
            }
            score *= dir_score+1;
            // look down
            dir_score = 0; k = i+1;
            while k < grid.len()-1 && grid[k][j] < this_tree {
                dir_score += 1;
                k += 1;
            }
            score *= dir_score+1;
            // update answer
            if score > answer { answer = score }
        }
    }

    println!("{answer}");
}

