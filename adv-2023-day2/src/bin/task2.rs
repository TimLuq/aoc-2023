use adv_2023_common::{SumTask, Task};

#[derive(Default)]
struct ParsedLine {
    power: u64,
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        let col = line.bytes().position(|b| b == b':').unwrap();
        let pulls = line[col + 2..].split(';');
        let mut max_red = 0u64;
        let mut max_green = 0u64;
        let mut max_blue = 0u64;
        for pull in pulls {
            let mut red = 0u64;
            let mut green = 0u64;
            let mut blue = 0u64;
            for pull_item in pull.split(',') {
                let pull_item = pull_item.trim();
                if let Some(n) = pull_item.strip_suffix(" red") {
                    red += n.parse::<u64>().unwrap();
                } else if let Some(n) = pull_item.strip_suffix(" green") {
                    green += n.parse::<u64>().unwrap();
                } else if let Some(n) = pull_item.strip_suffix(" blue") {
                    blue += n.parse::<u64>().unwrap();
                }
            }
            if red > max_red {
                max_red = red;
            }
            if green > max_green {
                max_green = green;
            }
            if blue > max_blue {
                max_blue = blue;
            }
        }
        let power = max_red * max_green * max_blue;
        ParsedLine { power }
    }
}

fn main() {
    let mut state = SumTask::new(|_, i| ParsedLine::parse(i), |s, l| s.sum += l.power);
    let res = state.run("adv-2023-day2/input/list.txt");
    println!("{}", res);
}
