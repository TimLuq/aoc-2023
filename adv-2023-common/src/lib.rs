use std::{io::{BufReader, BufRead}, fs::File, fmt::Display};


pub struct LineParser {
    line: String,
    file: BufReader<File>,
}

impl LineParser {
    pub fn new(path: &str) -> Self {
        Self {
            file: BufReader::new(File::open(path).unwrap()),
            line: String::with_capacity(64),
        }
    }

    fn next(&mut self) -> Option<&str> {
        self.line.clear();
        let len = self.file.read_line(&mut self.line).unwrap();
        if len == 0 {
            return None;
        }
        Some(&self.line)
    }

    pub fn process<S, L, P: Fn(&str) -> L, F: FnMut(&mut S, L)>(&mut self, parser: P, mut f: F, state: &mut S) {
        while let Some(line) = self.next() {
            let l = parser(line);
            f(state, l);
        }
    }
}

pub trait Task {
    type Input<'a> where Self: 'a;
    type Output<'a>: Display where Self: 'a;
    fn parse<'a>(&self, line: &'a str) -> Self::Input<'a>;
    fn process(&mut self, input: Self::Input<'_>);
    fn output(&self) -> Self::Output<'_>;
    fn run(&mut self, path: &str) -> Self::Output<'_> {
        let mut par = LineParser::new(path);
        while let Some(line) = par.next() {
            let input = self.parse(line);
            self.process(input);
        }
        self.output()
    }
}

pub struct SumTask<I> {
    pub sum: u64,
    p: fn(&SumTask<I>, &str) -> I,
    f: fn(&mut SumTask<I>, num: I),
}

impl<I> SumTask<I> {
    pub fn new(p: fn(&SumTask<I>, &str) -> I, f: fn(&mut SumTask<I>, num: I)) -> Self {
        Self {
            sum: 0,
            p,
            f,
        }
    }
}

impl<I: Into<u64>> SumTask<I> {
    pub fn add(slf: &mut Self, num: I) {
        slf.sum += num.into();
    }
}

impl<I: 'static> Task for SumTask<I> {
    type Input<'a> = I;
    type Output<'a> = u64;

    fn parse<'a>(&self, line: &'a str) -> Self::Input<'a> {
        (self.p)(self, line)
    }

    fn process(&mut self, input: Self::Input<'_>) {
        (self.f)(self, input);
    }

    fn output(&self) -> Self::Output<'_> {
        self.sum
    }
}

