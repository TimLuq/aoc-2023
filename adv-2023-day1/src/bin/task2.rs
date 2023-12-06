use adv_2023_common::{SumTask, Task};

struct ParsedLine {
    pub fst: u64,
    pub lst: u64,
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        let mut found = false;
        let mut fst = 0;
        let mut lst = 0;
        let mut line = line.as_bytes();
        while !line.is_empty() {
            for (word, num) in NUMBERS {
                if line.starts_with(word) {
                    if !found {
                        fst = num * 10;
                        found = true;
                    }
                    lst = num;
                    break;
                }
            }
            line = &line[1..];
        }
        Self { fst, lst }
    }
}

fn main() {
    let mut state = SumTask::new(|_, i| ParsedLine::parse(i), |s, l| s.sum += l.fst + l.lst);
    let res = state.run("adv-2023-day1/input/list.txt");
    println!("{}", res);
}

static NUMBERS: [(&[u8], u64); 19] = [
    (b"0", 0),
    (b"1", 1),
    (b"2", 2),
    (b"3", 3),
    (b"4", 4),
    (b"5", 5),
    (b"6", 6),
    (b"7", 7),
    (b"8", 8),
    (b"9", 9),
    //(b"zero", 0),
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
];
