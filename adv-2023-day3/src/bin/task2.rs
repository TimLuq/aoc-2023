use adv_2023_common::Task;

struct ParsedLine {
    pub nums: Vec<NumSpan>,
    pub gears: Vec<u32>,
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        ParsedLine {
            nums: Self::parse_num(line.as_bytes()),
            gears: Self::parse_gears(line.as_bytes()),
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
                nums.push(NumSpan {
                    val,
                    start: start as u32,
                    end: end as u32,
                });
            }
            i += 1;
        }
        if active {
            nums.push(NumSpan {
                val,
                start: start as u32,
                end: end as u32,
            });
        }
        nums
    }

    fn parse_gears(line: &[u8]) -> Vec<u32> {
        let mut syms = Vec::with_capacity(16);
        let mut i = 0;
        while i < line.len() {
            let c = line[i];
            if c != b'*' {
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
    pub gears: Vec<u32>,
    pub old_old_nums: Vec<NumSpan>,
    pub old_nums: Vec<NumSpan>,
    pub nums: Vec<NumSpan>,
    pub sum: u64,
}

impl State {
    fn compute(&mut self) {
        for val in core::mem::take(&mut self.gears) {
            let items: Vec<&[NumSpan]> = vec![&self.old_old_nums, &self.old_nums, &self.nums];
            self.sum += Self::adjacent(val, 0, items);
        }

        core::mem::swap(&mut self.old_old_nums, &mut self.old_nums);
        core::mem::swap(&mut self.old_nums, &mut self.nums);
        self.nums.clear();
    }
    fn adjacent(val: u32, acc: u64, mut nums: Vec<&[NumSpan]>) -> u64 {
        if nums.is_empty() {
            return acc;
        }
        let fst = nums[0];
        if fst.is_empty() || fst[0].start > val + 1 {
            nums.remove(0);
            return Self::adjacent(val, acc, nums);
        }
        if fst[0].end < val - 1 {
            nums[0] = &fst[1..];
            return Self::adjacent(val, acc, nums);
        }
        let base = fst[0].val;
        nums[0] = &fst[1..];
        let mul = Self::adjacent_pair(val, 0, nums.clone());
        Self::adjacent(val, acc + mul * base, nums)
    }
    fn adjacent_pair(val: u32, acc: u64, mut nums: Vec<&[NumSpan]>) -> u64 {
        if nums.is_empty() {
            return acc;
        }
        let fst = nums[0];
        if fst.is_empty() || fst[0].start > val + 1 {
            nums.remove(0);
            return Self::adjacent_pair(val, acc, nums);
        }
        if fst[0].end < val - 1 {
            nums[0] = &fst[1..];
            return Self::adjacent_pair(val, acc, nums);
        }
        let base = fst[0].val;
        nums[0] = &fst[1..];
        Self::adjacent_pair(val, acc + base, nums)
    }
}

impl Task for State {
    type Input<'a> = ParsedLine where Self: 'a;

    type Output<'a> = u64 where Self: 'a;

    fn parse<'a>(&self, line: &'a str) -> Self::Input<'a> {
        ParsedLine::parse(line)
    }

    fn process(&mut self, input: Self::Input<'_>) {
        self.nums = input.nums;
        self.compute();
        self.gears = input.gears;
    }

    fn output(&mut self) -> Self::Output<'_> {
        self.compute();
        self.sum
    }
}

fn main() {
    let mut state = State {
        nums: Vec::new(),
        old_old_nums: Vec::new(),
        old_nums: Vec::new(),
        gears: Vec::new(),
        sum: 0,
    };
    let res = state.run("adv-2023-day3/input/list.txt");
    println!("{}", res);
}
