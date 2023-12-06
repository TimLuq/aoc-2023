use adv_2023_common::Task;

enum ParsedLine {
    Times(Vec<u32>),
    Distances(Vec<u32>),
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        if line.starts_with("Time:") {
            return Self::parse_time(line);
        }
        if line.starts_with("Distance:") {
            return Self::parse_distance(line);
        }
        panic!("Unknown line: {:?}", line);
    }

    fn parse_time(line: &str) -> Self {
        let mut line = line[5..].trim();
        let mut nums = Vec::with_capacity(32);
        while let Some(pos) = line.find(' ') {
            nums.push(line[..pos].parse::<u32>().unwrap());
            line = line[pos + 1..].trim();
        }
        nums.push(line.parse::<u32>().unwrap());
        ParsedLine::Times(nums)
    }

    fn parse_distance(line: &str) -> Self {
        let mut line = line[9..].trim();
        let mut nums = Vec::with_capacity(32);
        while let Some(pos) = line.find(' ') {
            nums.push(line[..pos].parse::<u32>().unwrap());
            line = line[pos + 1..].trim();
        }
        nums.push(line.parse::<u32>().unwrap());
        ParsedLine::Distances(nums)
    }
}

#[derive(Debug, Default)]
struct State {
    times: Vec<u32>,
    distances: Vec<u32>,
}

impl State {
    #[inline]
    fn winning_strat_count(time: u32, distance: u32) -> u32 {
        // distance_d = time_hold * (time - time_hold)
        let mut res = 0;
        let mut i = 1;
        while i * (time - i) <= distance {
            i += 1;
        }
        while i * (time - i) > distance {
            res += 1;
            i += 1;
        }
        res
    }
}

impl Task for State {
    type Input<'a> = ParsedLine where Self: 'a;

    type Output<'a> = u64 where Self: 'a;

    fn parse<'a>(&self, line: &'a str) -> Self::Input<'a> {
        ParsedLine::parse(line)
    }

    fn process(&mut self, input: Self::Input<'_>) {
        match input {
            ParsedLine::Times(a) => {
                self.times = a;
            }
            ParsedLine::Distances(a) => {
                self.distances = a;
            }
        }
    }

    fn output(&mut self) -> Self::Output<'_> {
        let mut output = 1;
        for (time, distance) in self.times.iter().zip(&self.distances) {
            output *= Self::winning_strat_count(*time, *distance) as u64;
        }
        output
    }
}

fn main() {
    let mut state = State::default();
    let res = state.run("adv-2023-day6/input/list.txt");
    println!("{}", res);
}
