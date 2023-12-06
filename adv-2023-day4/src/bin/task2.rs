use std::collections::VecDeque;

use adv_2023_common::Task;

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

struct State {
    pub mul: VecDeque<u32>,
    pub agg: u64,
}

impl Task for State {
    type Input<'a> = ParsedLine where Self: 'a;

    type Output<'a> = u64 where Self: 'a;

    fn parse<'a>(&self, line: &'a str) -> Self::Input<'a> {
        ParsedLine::parse(line)
    }

    fn process(&mut self, input: Self::Input<'_>) {
        process(self, input);
    }

    fn output(&mut self) -> Self::Output<'_> {
        self.agg
    }
}

fn main() {
    let mut state = State {
        mul: VecDeque::new(),
        agg: 0,
    };
    let res = state.run("adv-2023-day4/input/list.txt");
    println!("{}", res);
}

fn process(task: &mut State, input: ParsedLine) {
    let mul = task.mul.pop_front().unwrap_or(0) + 1;
    task.agg += mul as u64;
    let mut agg = 0u32;
    let mut winning = input.winning.iter();
    let mut has = input.has.iter();
    let mut winning_cur = winning.next().unwrap();
    let mut has_cur = has.next().unwrap();
    loop {
        match winning_cur.cmp(has_cur) {
            std::cmp::Ordering::Equal => {
                agg += 1;
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
    for index in 0..(agg as usize) {
        if let Some(n) = task.mul.get_mut(index) {
            *n += mul;
        } else {
            task.mul.push_back(mul);
        }
    }
}
