//! Day 6 part 1 - Tuning Trouble

const SOP_LENGTH: usize = 4;

struct CircArray<'a, T> {
    arr: &'a mut [Option<T>],
    tail: usize,
}

impl<'a, T> CircArray<'a, T> where T: Eq {
    fn new(arr: &'a mut [Option<T>]) -> Self {
        CircArray { arr, tail: 0 }
    }

    fn push(&mut self, val: T) {
        self.arr[self.tail] = Some(val);
        self.tail = (self.tail+1) % self.arr.len();
    }

    //this could be way better but I already spent too much time learning generic lifetimes
    fn has_duplicates(&self) -> bool {
        for i in 0..self.arr.len()-1 {
            for j in i+1..self.arr.len() {
                if self.arr[i] == self.arr[j] { return true }
            }
        }
        false
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let line = buf.as_bytes();

    let mut ans = None;

    let mut recent_array: [Option<u8>; SOP_LENGTH] = [None; SOP_LENGTH];
    let mut circ = CircArray::new(&mut recent_array);
    
    //read first 4 elements
    let mut i = 0;
    while i < SOP_LENGTH {
        circ.push(line[i]);
        i+=1
    }

    while i < line.len() {
        circ.push(line[i]);
        i+=1;
        if !circ.has_duplicates() {
            ans = Some(i);
            break
        }
    }

    match ans {
        Some(n) => println!("{n}"),
        None => {
            eprintln!("Start-Of-Packet not found.");
            std::process::exit(1);
        },
    }
}

