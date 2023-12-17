use std::fmt::Debug;

use aoc_runner_derive::{aoc_generator, aoc};

type CoordType = u16;
type ValueType = u16;

type Point = crate::util::Point<CoordType>;

#[derive(Debug, Clone, Copy)]
pub enum Entry {
    Symbol(u8, Point),
    Number(ValueType, Point, Point),
}

impl Entry {
    pub const fn is_symbol(&self) -> bool { matches!(self, Self::Symbol(..)) }
    pub const fn is_number(&self) -> bool { matches!(self, Self::Number(..)) }

    pub const fn unwrap_symbol(self) -> (u8, Point) {
        match self {
            Self::Symbol(v, p) => (v, p),
            _ => panic!("Cannot unwrap Symbol from non-Symbol value.")
        }
    }

    pub const fn unwrap_number(self) -> (ValueType, (Point, Point)) {
        match self {
            Self::Number(v, p1, p2) => (v, (p1, p2)),
            _ => panic!("Cannot unwrap Symbol from non-Symbol value.")
        }
    }

    pub fn bounding_box(&self, max_height: CoordType, max_width: CoordType) -> (Point, Point) {
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
pub fn generator(input: &[u8]) -> (CoordType, CoordType, Vec<Entry>) {
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let height = input.iter().filter(|&&c| c == b'\n').count() + 1;

    let entries = input.split(|&char| char == b'\n')
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

                    let right = Point::from((idx - 1, line_idx));

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
        });

    (num_traits::cast(width).unwrap(), num_traits::cast(height).unwrap(), entries)
}

// I think it is?
#[derive(Debug, Clone)]
pub enum BoundingAreaHierarchy<T> {
    Node {
        // upper left, lower right
        bounding_area: (Point, Point),
        left: Box<BoundingAreaHierarchy<T>>,
        right: Box<BoundingAreaHierarchy<T>>,
    },
    Leaf {
        // upper left, lower right
        area: (Point, Point),
        value: T,
    },
    Empty
}

fn calc_bounding_area<T>(areas: &[(T, Area)]) -> Option<Area> {
    let x0 = areas.iter().map(|(_, (left, _))| left.x).min()?;
    let y0 = areas.iter().map(|(_, (left, _))| left.y).min()?;
    let x1 = areas.iter().map(|(_, (_, right))| right.x).max()?;
    let y1 = areas.iter().map(|(_, (_, right))| right.y).max()?;

    Some((Point::from((x0, y0)), Point::from((x1, y1))))
}

const fn contains_point(area: &Area, Point {x: x0, y: y0}: Point) -> bool {
    let &(Point {x: x1, y: y1}, Point {x: x2, y: y2}) = area;

    x1 <= x0 && y1 <= y0 &&
    x0 <= x2 && y0 <= y2
}


pub const fn intersects_area(area: &Area, (Point {x: x0, y: y0}, Point {x: x1, y: y1}): &Area) -> bool {
    contains_point(area, Point::new(*x0, *y0)) || 
    contains_point(area, Point::new(*x1, *y0)) || 
    contains_point(area, Point::new(*x0, *y1)) || 
    contains_point(area, Point::new(*x1, *y1))
}

type Area = (Point, Point);
impl<'b, T: Debug + Clone> BoundingAreaHierarchy<T> {
    pub const fn is_node(&self) -> bool { matches!(self, Self::Node { .. }) }
    pub const fn is_leaf(&self) -> bool { matches!(self, Self::Leaf{ .. }) }
    pub const fn is_empty(&self) -> bool { matches!(self, Self::Empty) }

    pub const fn contains(&self, point: Point) -> bool {
        let area = match self {
            Self::Node { bounding_area: area, .. } | Self::Leaf { area, .. } => area,
            _ => return false
        };

        contains_point(area, point)
    }

    pub const fn intersects(&self, (Point {x: x0, y: y0}, Point {x: x1, y: y1}): Area) -> bool {
        let area = match self {
            Self::Node { bounding_area: area, .. } | Self::Leaf { area, .. } => area,
            _ => return false
        };

        contains_point(area, Point::new(x0, y0)) || 
        contains_point(area, Point::new(x1, y0)) || 
        contains_point(area, Point::new(x0, y1)) || 
        contains_point(area, Point::new(x1, y1))
    }

    pub fn from_areas(mut areas: Vec<(T, Area)>) -> Self {
        Self::build_bah(&mut areas, 0)
    }
    fn build_bah(areas: &mut [(T, Area)], dim: usize) -> Self {
        fn area_dim_key<T>((_, (_, right)): &(T, Area), dim: usize) -> CoordType {
            if dim & 1 == 0 {
                right.x
            } else {
                right.y
            }
        }

        if areas.is_empty() {
            Self::Empty
        } else if areas.len() == 1 {
            let (value, area) = areas.first().unwrap();
            Self::Leaf { area: *area, value: value.clone() }
        } else {
            let bounding_area = calc_bounding_area(&areas).unwrap();

            // println!("L: {}  D: {}", areas.len(), if dim & 1 == 0 {'x'} else {'y'});
            // println!("{areas:?}");

            areas.sort_by_key(|a| area_dim_key(a, dim));
            let partition_value = area_dim_key(&areas[areas.len() / 2 - 1], dim);
            // println!("partition value: {partition_value}");

            let lower_count = areas.iter_mut().partition_in_place(|a| area_dim_key(a, dim) <= partition_value);

            Self::Node {
                bounding_area,
                left: Box::new(Self::build_bah(&mut areas[..lower_count], dim + 1)),
                right: Box::new(Self::build_bah(&mut areas[lower_count..], dim + 1))
            }
        }
    }

    pub fn contains_intersection(&self, target: &Area) -> bool {
        match self {
            Self::Node { bounding_area, left, right } if intersects_area(bounding_area, target) => {
                left.contains_intersection(target) ||
                right.contains_intersection(target)
            },
            Self::Leaf { area, .. } if intersects_area(area, &target) => {
                true
            }
            _ => false,
        }
    }

    pub fn search_intersections(&self, target: &Area) -> Vec<&T> {
        let mut buffer = Vec::with_capacity(1);
        self.recursive_intersection_search(target, &mut buffer);
        return buffer;
    }

    fn recursive_intersection_search(&'b self, target: &Area, intersections: &mut Vec<&'b T>) {
        match self {
            Self::Node { bounding_area, left, right } if intersects_area(bounding_area, target) => {
                left.recursive_intersection_search(target, intersections);
                right.recursive_intersection_search(target, intersections);
            },
            Self::Leaf { area, value } if intersects_area(&area, target) =>  {
                println!("{area:?} ({value:?}) contains {target:?}");
                intersections.push(value)
            }
            _ => (),
        }
    }
}

impl<V: Debug + Clone> FromIterator<(V, Area)> for BoundingAreaHierarchy<V> {
    fn from_iter<T: IntoIterator<Item = (V, Area)>>(iter: T) -> Self {
        Self::from_areas(iter.into_iter().collect())
    }
}

fn expand_area<const E: CoordType>((tl, br): Area, max_height: CoordType, max_width: CoordType) -> Area {
    let left = tl.x.checked_sub(E).unwrap_or(0);
    let top = tl.y.checked_sub(E).unwrap_or(0);
    let right = max_width.min(br.x + E);
    let bot = max_height.min(br.y + E);

    (Point::new(left, top), Point::new(right, bot))
}

fn expand_point<const E: CoordType>(p: Point, max_height: CoordType, max_width: CoordType) -> Area {
    expand_area::<E>((p, p), max_height, max_width)
}

#[aoc(day3, part1)]
fn solver_part1((width, height, entries): &(CoordType, CoordType, Vec<Entry>)) -> u32 {
    let (numbers, symbols) = entries.iter().copied().partition::<Vec<_>, _>(Entry::is_number);
    let numbers = numbers.into_iter().map(Entry::unwrap_number).collect::<Vec<_>>();
    let symbols = symbols.into_iter().map(Entry::unwrap_symbol).collect::<Vec<_>>();

    let hiearchy = symbols
        .iter()
        .map(|&(c, p)| (c as char, expand_point::<1>(p, *height, *width)))
        .collect::<BoundingAreaHierarchy<_>>();

    let found_numbers = numbers.iter()
        .filter(|(_, a)| hiearchy.contains_intersection(a))
        .collect::<Vec<_>>();

    let mut masked_output = (0..*height).map(|_| ".".repeat(*width as usize)).collect::<Vec<_>>();

    found_numbers.iter().for_each(|(n, (p, _))| {
        let y = p.y as usize;
        let x = p.x as usize;
        let s = n.to_string();
        masked_output[y].replace_range(x..x + s.len(), &s);
    });

    symbols.iter().for_each(|&(c, p)| {
        let y = p.y as usize;
        let x = p.x as usize;
        unsafe {
            masked_output[y].as_bytes_mut()[x] = c;
        }
    });

    masked_output.into_iter().for_each(|l| println!("{}", l));

    found_numbers.into_iter()
        .map(|(v, ..)| v)
        .sum::<u16>() as u32
}
