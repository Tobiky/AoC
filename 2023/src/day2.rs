use aoc_runner_derive::{aoc_generator, aoc};


#[derive(Clone, Copy)]
pub struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    pub fn new() -> Self {
        Hand { red: 0, green: 0, blue: 0 }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

pub struct Game {
    attempts: Vec<Hand>,
}

impl Game {
    pub fn with_hands(hands: Vec<Hand>) -> Self {
        Self {
            attempts: hands
        }
    }
}

#[aoc_generator(day2)]
pub fn generator(input: &[u8]) -> Vec<(u32, Game)> {
    fn parse_hand(input: &[u8]) -> Hand {
        let mut hand = Hand::new();
        let mut idx = 1;
        while idx < input.len() {
            // index of space since idx
            let space_idx = input[idx..].iter().position(|&c| c == b' ').unwrap();

            // number between idx and idx of space
            let number = std::str::from_utf8(&input[idx..idx + space_idx]).unwrap().parse().unwrap();

            // space and include starting character
            idx += space_idx + 1;
            match input[idx] {
                b'r' => {
                    hand.red = number;
                    idx += "red".len();
                },
                b'g' => {
                    hand.green = number;
                    idx += "green".len();
                },
                b'b' => {
                    hand.blue= number;
                    idx += "blue".len();
                },
                _ => panic!("Unaccepted character '{}' in position {} of substring {}", input[idx] as char, idx, std::str::from_utf8(input).unwrap()),
            };
            // comma space
            idx += 2;
        }

        hand
    }

    input.split(|&char| char == b'\n')
        .map(|slice| &slice[slice.iter().position(|&char| char == b':').unwrap() + 1..])
        .map(|slice| slice.split(|&char| char == b';').map(parse_hand).collect::<Vec<_>>())
        .map(Game::with_hands)
        .enumerate()
        .map(|(idx, game)| (idx as u32 + 1, game))
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
pub fn part1_solver(input: &[(u32, Game)]) -> u32 {
    input.iter()
        .filter(|(_, game)| game.attempts.iter().all(|hand| hand.red <= 12 && hand.green <= 13 && hand.blue <= 14))
        .map(|(id, _)| id)
        .sum()
}


#[aoc(day2, part2)]
pub fn part2_solver(input: &[(u32, Game)]) -> u32 {
    input.iter()
        .map(|(_, game)| game.attempts.iter().copied().fold(Hand::new(), Hand::max))
        .map(|hand| hand.red * hand.green * hand.blue)
        .sum()
}
