use adv_2023_common::Task;

enum ParsedLine {
    Time(u64),
    Distance(u64),
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
        let mut num = 0;
        while let Some(pos) = line.find(' ') {
            let num_part = &line[..pos];
            num = num * 10u64.pow(num_part.len() as u32) + num_part.parse::<u64>().unwrap();
            line = line[pos + 1..].trim();
        }
        num = num * 10u64.pow(line.len() as u32) + line.parse::<u64>().unwrap();
        ParsedLine::Time(num)
    }

    fn parse_distance(line: &str) -> Self {
        let mut line = line[9..].trim();
        let mut num = 0;
        while let Some(pos) = line.find(' ') {
            let num_part = &line[..pos];
            num = num * 10u64.pow(num_part.len() as u32) + num_part.parse::<u64>().unwrap();
            line = line[pos + 1..].trim();
        }
        num = num * 10u64.pow(line.len() as u32) + line.parse::<u64>().unwrap();
        ParsedLine::Distance(num)
    }
}

#[derive(Debug, Default)]
struct State {
    time: u64,
    distance: u64,
}

impl State {
    #[inline]
    fn winning_strat_count(time: u64, distance: u64) -> u64 {
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
            ParsedLine::Time(a) => {
                self.time = a;
            }
            ParsedLine::Distance(a) => {
                self.distance = a;
            }
        }
    }

    fn output(&mut self) -> Self::Output<'_> {
        Self::winning_strat_count(self.time, self.distance)
    }
}

fn main() {
    let mut state = State::default();
    let res = state.run("adv-2023-day6/input/list.txt");
    println!("{}", res);
}
