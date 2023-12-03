use std::collections::HashMap;

use debug_print::debug_println;
use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn solver_part1(input: &[u8]) -> u32 {
    input
        .split(|&x| x == b'\n')
        .map(|x| {
            let i = x
                .iter()
                .copied()
                .position(|x| b'0' <= x && x <= b'9')
                .unwrap();
            let first = x[i];
            let last = x
                .iter()
                .copied()
                .skip(i)
                .filter(|&x| b'0' <= x && x <= b'9')
                .last()
                .unwrap();

            ((first - b'0') * 10 + (last - b'0')) as u32
        })
        .sum()
}

// words: one, two, three, four, five, six, seven, eight, nine

struct PrefixTree {
    value: Option<u32>,
    trees: HashMap<u8, PrefixTree>,
}

impl PrefixTree {
    pub fn empty() -> Self {
        Self {
            value: None,
            trees: HashMap::new(),
        }
    }

    pub fn insert(&mut self, sequence: &[u8], value: u32) {
        if sequence.is_empty() {
            self.value = Some(value);
        } else if self.trees.contains_key(&sequence[0]) {
            self.trees.get_mut(&sequence[0])
                .unwrap()
                .insert(&sequence[1..], value);
        } else {
            let mut subtree = Self::empty();
            subtree.insert(&sequence[1..], value);
            self.trees.insert(sequence[0], subtree);
        }
    }

    pub fn create() -> Self {
        let mut tree = Self::empty();

        [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .into_iter()
        .enumerate()
        .for_each(|(idx, word)| tree.insert(word.as_bytes(), idx as u32 + 1));

        (b'1'..=b'9')
            .into_iter()
            .enumerate()
            .for_each(|(idx, word)| tree.insert(&[word], idx as u32 + 1));

        tree
    }

    // (value, search length)
    pub fn find(&self, sequence: &[u8]) -> Option<(u32, usize)> {
        if let Some(value) = self.value {
            Some((value, 0))
        } else if let Some((value, length)) = sequence
            .get(0)
            .and_then(|c| self.trees.get(c))
            .and_then(|t| t.find(&sequence[1..]))
        {
            Some((value, length + 1))
        } else {
            None
        }
    }
}

#[aoc(day1, part2)]
pub fn solver_part2(input: &[u8]) -> u32 {
    let tree = PrefixTree::create();


    input
        .split(|&x| x == b'\n')
        .enumerate()
        .map(|(_idx, x)| {
            println!("line {} ({})", _idx + 1, std::str::from_utf8(x).unwrap());
            let mut first = 0;
            let mut last = 0;

            let mut i = 0;
            while i < x.len() {
                if let Some((digit, length)) = tree.find(&x[i..]) {
                    first = digit;
                    println!(
                        "first <- {} '{:}'  \t{:?}",
                        digit,
                        std::str::from_utf8(&x[i..i + length]).unwrap(),
                        (i+1, i+length+1)
                    );
                    i += 1;
                    break;
                }
                i += 1;
            }
            while i < x.len() {
                if let Some((digit, length)) = tree.find(&x[i..]) {
                    println!(
                        "last  <- {} '{}'  \t{:?}",
                        digit,
                        std::str::from_utf8(&x[i..i + length]).unwrap(),
                        (i+1, i+length+1)
                    );
                    last = digit;
                    i += 1;
                } else {
                    i += 1;
                }
            }

            let result = if last == 0 {first} else {first * 10 + last};
            println!("result: {result}\n");
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    mod prefix_tree {
        use crate::day1::PrefixTree;

        macro_rules! digit_test {
            ($name:ident, $text:tt, $num:literal) => {
                #[test]
                fn $name() {
                    let _match = PrefixTree::create().find(stringify!($text).as_bytes());
                    assert!(_match.is_some());
                    assert!(matches!(_match, Some(($num, _))));
                }
            };
            ($text:ident, $num:literal) => {
                digit_test!($text, $text, $num);
            };
        }

        digit_test!(one, 1);
        digit_test!(two, 2);
        digit_test!(three, 3);
        digit_test!(four, 4);
        digit_test!(five, 5);
        digit_test!(six, 6);
        digit_test!(seven, 7);
        digit_test!(eight, 8);
        digit_test!(nine, 9);

        digit_test!(digit_1, 1, 1);
        digit_test!(digit_2, 2, 2);
        digit_test!(digit_3, 3, 3);
        digit_test!(digit_4, 4, 4);
        digit_test!(digit_5, 5, 5);
        digit_test!(digit_6, 6, 6);
        digit_test!(digit_7, 7, 7);
        digit_test!(digit_8, 8, 8);
        digit_test!(digit_9, 9, 9);

        #[test]
        fn six_detail() {
            let mut tree = &PrefixTree::create();

            assert!(tree.trees.contains_key(&b's'));
            tree = &tree.trees[&b's'];

            assert!(tree.trees.contains_key(&b'i'));
            tree = &tree.trees[&b'i'];

            assert!(tree.trees.contains_key(&b'x'));
            tree = &tree.trees[&b'x'];

            assert!(tree.value.is_some());

            assert_eq!(tree.value, Some(6));
        }
    }
}
