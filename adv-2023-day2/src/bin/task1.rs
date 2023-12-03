use adv_2023_common::{SumTask, Task};

#[derive(Default)]
struct ParsedLine {
    game: u64,
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        let col = line.bytes().position(|b| b == b':').unwrap();
        let game = line[5..col].parse::<u64>().unwrap();
        let pulls = line[col + 2..].split(';');
        for pull in pulls {
            let mut red = 0u16;
            let mut green = 0u16;
            let mut blue = 0u16;
            for pull_item in pull.split(',') {
                let pull_item = pull_item.trim();
                if let Some(n) = pull_item.strip_suffix(" red") {
                    red += n.parse::<u16>().unwrap();
                    if red > RGB_FILTER.0 {
                        return Default::default();
                    }
                } else if let Some(n) = pull_item.strip_suffix(" green") {
                    green += n.parse::<u16>().unwrap();
                    if green > RGB_FILTER.1 {
                        return Default::default();
                    }
                } else if let Some(n) = pull_item.strip_suffix(" blue") {
                    blue += n.parse::<u16>().unwrap();
                    if blue > RGB_FILTER.2 {
                        return Default::default();
                    }
                }
            }
        }
        ParsedLine { game }
    }
}

fn main() {
    let mut state = SumTask::new(|_, i| ParsedLine::parse(i), |s, l| s.sum += l.game);
    let res = state.run("adv-2023-day2/input/list.txt");
    println!("{}", res);
}

static RGB_FILTER: (u16, u16, u16) = (12, 13, 14);
