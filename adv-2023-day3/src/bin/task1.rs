use adv_2023_common::Task;

struct ParsedLine {
    pub nums: Vec<NumSpan>,
    pub syms: Vec<u32>,
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        ParsedLine {
            nums: Self::parse_num(line.as_bytes()),
            syms: Self::parse_sym(line.as_bytes()),
        }
    }

    fn parse_num(line: &[u8]) -> Vec<NumSpan> {
        let mut active = false;
        let mut nums = Vec::with_capacity(16);
        let mut val = 0;
        let mut start = 0;
        let mut end = 0;
        let mut i = 0;
        while i < line.len() {
            let c = line[i];
            if c.is_ascii_digit() {
                if active {
                    end = i;
                    val = val * 10 + (c - b'0') as u64;
                } else {
                    active = true;
                    start = i;
                    end = i;
                    val = (c - b'0') as u64;
                }
            } else if active {
                active = false;
                nums.push(NumSpan { val, start: start as u32, end: end as u32 });
            }
            i += 1;
        }
        if active {
            nums.push(NumSpan { val, start: start as u32, end: end as u32 });
        }
        nums
    }

    fn parse_sym(line: &[u8]) -> Vec<u32> {
        let mut syms = Vec::with_capacity(16);
        let mut i = 0;
        while i < line.len() {
            let c = line[i];
            if c.is_ascii_digit() || c == b'.' {
                i += 1;
                continue;
            }
            syms.push(i as u32);
            i += 1;
        }
        syms
    }
}

struct NumSpan {
    pub val: u64,
    pub start: u32,
    pub end: u32,
}

struct State {
    pub nums: Vec<NumSpan>,
    pub old_old_sym: Vec<u32>,
    pub old_sym: Vec<u32>,
    pub sym: Vec<u32>,
    pub sum: u64,
}

impl State {
    fn compute(&mut self) {
        for num in core::mem::take(&mut self.nums) {
            let val = num.val;
            let find_after = if num.start == 0 { 0 } else { num.start - 1 };
            let find_before = num.end + 1;
            if self.matches(find_after, find_before) {
                self.sum += val;
            }
        }

        core::mem::swap(&mut self.old_old_sym, &mut self.old_sym);
        core::mem::swap(&mut self.old_sym, &mut self.sym);
        self.sym.clear();
    }
    fn matches(&self, find_after: u32, find_before: u32) -> bool {
        self.old_old_sym.iter().any(|&s| {
            s >= find_after && s <= find_before
        }) || self.old_sym.iter().any(|&s| {
            s >= find_after && s <= find_before
        }) || self.sym.iter().any(|&s| {
            s >= find_after && s <= find_before
        })
    }
}

impl Task for State {
    type Input<'a> = ParsedLine where Self: 'a;

    type Output<'a> = u64 where Self: 'a;

    fn parse<'a>(&self, line: &'a str) -> Self::Input<'a> {
        ParsedLine::parse(line)
    }

    fn process(&mut self, input: Self::Input<'_>) {
        self.sym = input.syms;
        self.compute();
        self.nums = input.nums;
    }

    fn output(&mut self) -> Self::Output<'_> {
        self.compute();
        self.sum
    }
}

fn main() {
    let mut state = State {
        nums: Vec::new(),
        old_old_sym: Vec::new(),
        old_sym: Vec::new(),
        sym: Vec::new(),
        sum: 0,
    };
    let res = state.run("adv-2023-day3/input/list.txt");
    println!("{}", res);
}
