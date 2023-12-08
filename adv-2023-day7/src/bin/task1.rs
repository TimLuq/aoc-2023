use std::collections::BTreeMap;

use adv_2023_common::Task;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
#[repr(u8)]
enum Card {
    N2 = 0,
    N3 = 1,
    N4 = 2,
    N5 = 3,
    N6 = 4,
    N7 = 5,
    N8 = 6,
    N9 = 7,
    T = 8,
    J = 9,
    Q = 10,
    K = 11,
    A = 12,
}

struct ParsedLine {
    hand: [Card; 5],
    bid: u32,
}

impl ParsedLine {
    pub fn parse(line: &str) -> Self {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid = bid.trim().parse::<u32>().unwrap();
        let mut hand = hand.chars().map(|c| match c {
            '2' => Card::N2,
            '3' => Card::N3,
            '4' => Card::N4,
            '5' => Card::N5,
            '6' => Card::N6,
            '7' => Card::N7,
            '8' => Card::N8,
            '9' => Card::N9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Invalid card: {:?}", c),
        });
        let hand = [
            hand.next().unwrap(),
            hand.next().unwrap(),
            hand.next().unwrap(),
            hand.next().unwrap(),
            hand.next().unwrap(),
        ];
        ParsedLine { hand, bid }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
#[repr(u8)]
enum HandType {
    HighCard = 0,
    Pair = 1,
    TwoPairs = 2,
    Three = 3,
    FullHouse = 4,
    Four = 5,
    Five = 6,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
struct Hand(HandType, Card, Card, Card, Card, Card);

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        let hand_type = Self::hand_type(&cards);
        Hand(hand_type, cards[0], cards[1], cards[2], cards[3], cards[4])
    }

    fn hand_type(cards: &[Card; 5]) -> HandType {
        let mut counts: [u8; 13] = [0; 13];
        for &card in cards {
            counts[card as u8 as usize] += 1;
        }
        counts.sort_by(|c1, c2| c2.cmp(c1));
        let mut counts = counts.into_iter();
        let count1 = counts.next().unwrap();
        if count1 == 5 {
            return HandType::Five;
        }
        if count1 == 4 {
            return HandType::Four;
        }
        let count2 = counts.next().unwrap();
        if count1 == 3 && count2 == 2 {
            return HandType::FullHouse;
        }
        if count1 == 3 {
            return HandType::Three;
        }
        if count1 == 2 && count2 == 2 {
            return HandType::TwoPairs;
        }
        if count1 == 2 {
            return HandType::Pair;
        }
        HandType::HighCard
    }
}

#[derive(Debug, Default)]
struct State {
    hands: BTreeMap<Hand, u32>,
}

impl State {
    fn ranks(&self) -> impl Iterator<Item = (usize, u32)> + '_ {
        self.hands
            .iter()
            .enumerate()
            .map(|(n, (_, bid))| (n + 1, *bid))
    }
}

impl Task for State {
    type Input<'a> = ParsedLine where Self: 'a;

    type Output<'a> = u64 where Self: 'a;

    fn parse<'a>(&self, line: &'a str) -> Self::Input<'a> {
        ParsedLine::parse(line)
    }

    fn process(&mut self, input: Self::Input<'_>) {
        let ParsedLine { hand, bid } = input;
        let hand = Hand::new(hand);
        match self.hands.entry(hand) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                //eprintln!("New hand: {:?} {:?}", hand, bid);
                entry.insert(bid);
            }
            std::collections::btree_map::Entry::Occupied(entry) => {
                panic!("Duplicate hand: {:?} {:?} {:?}", hand, entry.get(), bid);
            }
        }
    }

    fn output(&mut self) -> Self::Output<'_> {
        self.ranks().map(|(n, bid)| n as u64 * bid as u64).sum()
    }
}

fn main() {
    let mut state = State::default();
    let res = state.run("adv-2023-day7/input/list.txt");
    println!("{}", res);
}
