use aoc22::line_reader::LineReader;

fn get_letter_priority(ascii_letter: u8) -> usize {
    if ascii_letter <= 0x5A { // A-Z (0x41-0x5A)
        (ascii_letter as usize) - 0x40 + 26
    }
    else { // a-z (0x61-0x7A)
        (ascii_letter as usize) - 0x60
    }
}

fn main() {
    let mut lr = LineReader::new();
    let mut answer = 0;

    let mut elf_no = 0;
    let mut sacks = [[0; 26*2]; 2];
    while lr.read_next().unwrap() > 0 {
        let items = lr.line.as_bytes(); // only chars in the A-Za-z range

        if elf_no < 2 {
            for c in items {
                let p = get_letter_priority(*c);
                sacks[elf_no][p-1] += 1;
            }
        }
        else {
            for c in items {
                let p = get_letter_priority(*c);
                if sacks[0][p-1] > 0 && sacks[1][p-1] > 0 {
                    answer += p;
                    sacks = [[0; 26*2]; 2]; //reset sacks
                    break;
                }
            }
        }

        elf_no = (elf_no + 1)%3
    }

    println!("Answer: {}", answer)
}

