use aoc_runner_derive::aoc_generator;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point {x, y}
    }
}

pub enum Entry {
    Symbol(u8, Point),
    Number(u32, Point, Point),
}

impl Entry {
    pub fn is_symbol(&self) -> bool { matches!(self, Self::Symbol(..)) }
    pub fn is_number(&self) -> bool { matches!(self, Self::Number(..)) }
    pub fn bounding_box(&self, max_height: usize, max_width: usize) -> (Point, Point) {
        let left = match self {
            Entry::Symbol(_, p) | Entry::Number(_, p, _) => p,
        };

        let right = match self {
            Entry::Symbol(_, p) | Entry::Number(_, _, p) => p,
        };

        let upper_left = Point {
            x: left.x.checked_sub(1).unwrap_or(0),
            y: left.y.checked_sub(1).unwrap_or(0),
        };

        let lower_right = Point {
            x: max_width.min(right.x + 1),
            y: max_height.min(right.y + 1),
        };

        (upper_left, lower_right)
    }
}

#[aoc_generator(day3)]
pub fn generator(input: &[u8]) -> Vec<Entry> {
    input.split(|&char| char == b'\n')
        .enumerate()
        .fold(Vec::new(), |mut entries, (line_idx, line)| {
            let mut idx = 0;

            while idx < line.len() {
                // parse a sequence of digits into Entry::Number
                if line[idx].is_ascii_digit() {
                    let number_length = line[idx..].iter().copied().take_while(u8::is_ascii_digit).count();
                    let number = std::str::from_utf8(&line[idx..idx + number_length]).unwrap().parse().unwrap();

                    let left = Point::from((idx, line_idx));

                    idx += number_length;

                    let right = Point::from((idx, line_idx));

                    entries.push(Entry::Number(number, left, right));
                }
                // parse non-dot symbol into Entry::Symbol
                else if line[idx] != b'.' {
                    entries.push(Entry::Symbol(line[idx], Point::from((idx, line_idx))));
                    idx += 1;
                }
                // dot; skip
                else {
                    idx += 1;
                }
            }

            entries
        })
}
