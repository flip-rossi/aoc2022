//! Day 20: Grove Positioning System

fn main() {
    // Parse input
    let numbers = std::io::stdin().lines()
        .map(|s| s.unwrap().trim().parse().unwrap())
        .enumerate()
        .collect::<Vec<(usize,i32)>>();

    // Solve
    let answer = aoc22::solve_puzzle!(numbers);
    println!("{answer}")
}

fn circ_right_index(i: usize, len: usize) -> usize {
    (i + 1) % len
}

fn circ_left_index(i: usize, len: usize) -> usize {
    if i > 0 {
        i - 1
    } else {
        len - 1
    }
}

//=============== PART 1 ===============//
fn part1(mut nums: Vec<(usize,i32)>) -> i64 {
    let nlen = nums.len();
    let mut new_indexes = (0..nlen).collect::<Vec<usize>>();

    for original_i in 0..new_indexes.len() {
        let i = new_indexes[original_i];
        let n = nums[i].1;
        if n >= 0 {
            // shift right
            let mut prev = i;
            let mut next = circ_right_index(i, nlen);
            for _ in 0..n {
                new_indexes[nums[prev].0] = next;
                new_indexes[nums[next].0] = prev;
                nums.swap(prev, next);
                prev = next;
                next = circ_right_index(next, nlen);
            }
        } else {
            // shift left
            let mut prev = i;
            let mut next = circ_left_index(i, nlen);
            for _ in 0..n.abs() {
                new_indexes[nums[prev].0] = next;
                new_indexes[nums[next].0] = prev;
                nums.swap(prev, next);
                prev = next;
                next = circ_left_index(next, nlen);
            }
        }
    }

    let zero_ind = nums.iter().enumerate()
        .find_map(|(i,n)| match n {
            (_,0) => Some(i),
            _ => None
        }).unwrap();

    (nums[(zero_ind + 1000)%nlen].1 + nums[(zero_ind + 2000)%nlen].1 + nums[(zero_ind + 3000)%nlen].1) as i64
}

//=============== PART 2 ===============//
const KEY: i64 = 811589153;

fn part2(nums: Vec<(usize,i32)>) -> i64 {
    let nlen = nums.len();
    let mut nums = nums.iter()
        .map(|(i,n)| (*i, *n as i64 * KEY))
        .collect::<Vec<_>>();
    let mut new_indexes = (0..nlen).collect::<Vec<_>>();

    for original_i in 0..new_indexes.len()*10 {
        let i = new_indexes[original_i % new_indexes.len()];
        let new_pos = (i as i64 + nums[i].1) % (nlen-1) as i64;
        let new_ind = if new_pos < 0 {
            (new_pos + nlen as i64 -1) as usize
        }
        else {
            new_pos as usize
        };
        let moved_val = nums[i];
        nums.remove(i);
        nums.insert(new_ind, moved_val);
        new_indexes[nums[new_ind].0] = new_ind;
        if new_ind > i {
            for j in i..new_ind {
                new_indexes[nums[j].0] -= 1;
            }
        }
        else if new_ind < i {
            for j in new_ind+1..=i {
                new_indexes[nums[j].0] += 1;
            }
        }
    }

    let zero_ind = nums.iter().enumerate()
        .find_map(|(i,n)| match n {
            (_,0) => Some(i),
            _ => None
        }).unwrap();

    eprintln!("  {}", nums[(zero_ind + 1000)%nlen].1);
    eprintln!("+ {}", nums[(zero_ind + 2000)%nlen].1);
    eprintln!("+ {}\n", nums[(zero_ind + 3000)%nlen].1);
    nums[(zero_ind + 1000)%nlen].1
        + nums[(zero_ind + 2000)%nlen].1
        + nums[(zero_ind + 3000)%nlen].1
}

