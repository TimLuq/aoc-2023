use adv_2023_common::{Task, SumTask};

struct ParsedLine {
    pub winning: Vec<u16>,
    pub has: Vec<u16>,
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        let pos = line.bytes().position(|b| b == b':').unwrap();
        let (winning, has) = line[pos + 1..].split_once('|').unwrap();
        ParsedLine {
            winning: Self::parse_num(winning.trim()),
            has: Self::parse_num(has.trim()),
        }
    }

    fn parse_num(mut line: &str) -> Vec<u16> {
        let mut nums = Vec::with_capacity(32);
        loop {
            line = line.trim_start();
            if line.is_empty() {
                nums.sort_unstable();
                return nums;
            }
            let (num, rest) = if let Some((a, b)) = line.split_once(' ') {
                (a, b)
            } else {
                (line, "")
            };
            line = rest;
            nums.push(num.parse::<u16>().unwrap());
        }
    }
}

fn main() {
    let mut state = SumTask::new(|_, i| ParsedLine::parse(i), process);
    let res = state.run("adv-2023-day4/input/list.txt");
    println!("{}", res);
}

fn process(task: &mut SumTask<ParsedLine>, input: ParsedLine) {
    let mut agg = 0u32;
    let mut winning = input.winning.iter();
    let mut has = input.has.iter();
    let mut winning_cur = winning.next().unwrap();
    let mut has_cur = has.next().unwrap();
    loop {
        match winning_cur.cmp(has_cur) {
            std::cmp::Ordering::Equal => {
                if agg == 0 {
                    agg = 1;
                } else {
                    agg *= 2;
                }
                if let Some(v) = has.next() {
                    has_cur = v;
                } else {
                    break;
                }
            }
            std::cmp::Ordering::Less => {
                if let Some(v) = winning.next() {
                    winning_cur = v;
                } else {
                    break;
                }
            }
            std::cmp::Ordering::Greater => {
                if let Some(v) = has.next() {
                    has_cur = v;
                } else {
                    break;
                }
            }
        }
    }
    task.sum += agg as u64;
}
