use aoc_runner_derive::aoc_generator;


pub struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl Hand {
    pub fn new() -> Self {
        Hand { red: 0, green: 0, blue: 0 }
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
pub fn generator(input: &[u8]) -> Vec<(usize, Game)> {
    fn parse_hand(input: &[u8]) -> Hand {
        let mut hand = Hand::new();
        let mut idx = 1;
        while idx < input.len() {
            let space_idx = input.iter().position(|&c| c == b' ').unwrap();
            let number = std::str::from_utf8(&input[..space_idx]).unwrap().parse().unwrap();
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
        .map(|(idx, game)| (idx + 1, game))
        .collect::<Vec<_>>()
}
