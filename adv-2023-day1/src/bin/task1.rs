use adv_2023_common::{SumTask, Task};

struct ParsedLine {
    pub fst: u64,
    pub lst: u64
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        let mut found = false;
        let mut fst = 0;
        let mut lst = 0;
        for c in line.bytes() {
            if c.is_ascii_digit() {
                if !found {
                    fst = (c - b'0') as u64 * 10;
                    found = true;
                }
                lst = c - b'0';
            }
        }
        Self {
            fst,
            lst: lst as u64
        }
    }
}

fn main() {
    let mut state = SumTask::new(|_, i| ParsedLine::parse(i), |s, l| s.sum += l.fst + l.lst);
    let res = state.run("adv-2023-day1/input/list.txt");
    println!("{}", res);
}
