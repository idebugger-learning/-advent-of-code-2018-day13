use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, PartialEq, Eq)]
enum RailSegment {
    Horizontal,
    Vertical,
    CurveRight,
    CurveLeft,
    Intersection,
    Empty,
}

impl Display for RailSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RailSegment::Horizontal => write!(f, "-"),
            RailSegment::Vertical => write!(f, "|"),
            RailSegment::CurveRight => write!(f, "/"),
            RailSegment::CurveLeft => write!(f, "\\"),
            RailSegment::Intersection => write!(f, "+"),
            RailSegment::Empty => write!(f, " "),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone)]
pub struct Cart {
    direction: Direction,
    next_turn: Turn,
}

impl Display for Cart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Up => write!(f, "^"),
        }
    }
}

pub struct RailRoad {
    map: Vec<Vec<RailSegment>>,
    pub carts: HashMap<(usize, usize), Cart>,
    pub crushed_carts: HashSet<(usize, usize)>,
}

impl RailRoad {
    pub fn new_from_str(str: &str) -> RailRoad {
        let mut carts = HashMap::new();
        let map = str
            .split("\n")
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, chr)| match chr {
                        ' ' => RailSegment::Empty,
                        '-' => RailSegment::Horizontal,
                        '|' => RailSegment::Vertical,
                        '/' => RailSegment::CurveRight,
                        '\\' => RailSegment::CurveLeft,
                        '+' => RailSegment::Intersection,
                        '>' => {
                            carts.insert(
                                (x, y),
                                Cart {
                                    direction: Direction::Right,
                                    next_turn: Turn::Left,
                                },
                            );
                            RailSegment::Horizontal
                        }
                        '<' => {
                            carts.insert(
                                (x, y),
                                Cart {
                                    direction: Direction::Left,
                                    next_turn: Turn::Left,
                                },
                            );
                            RailSegment::Horizontal
                        }
                        '^' => {
                            carts.insert(
                                (x, y),
                                Cart {
                                    direction: Direction::Up,
                                    next_turn: Turn::Left,
                                },
                            );
                            RailSegment::Vertical
                        }
                        'v' => {
                            carts.insert(
                                (x, y),
                                Cart {
                                    direction: Direction::Down,
                                    next_turn: Turn::Left,
                                },
                            );
                            RailSegment::Vertical
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        RailRoad {
            map,
            carts,
            crushed_carts: HashSet::new(),
        }
    }

    pub fn tick(&mut self) {
        let mut sorted_carts = self.carts.keys().map(|&coords| coords).collect::<Vec<_>>();
        sorted_carts.sort_unstable_by(|&a, &b| match a.1.cmp(&b.1) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
        });

        sorted_carts.iter().for_each(|&coords| {
            self.move_cart(coords);
        });
    }

    pub fn get_display_size(&self) -> usize {
        self.map.len()
    }

    fn move_cart(&mut self, coords: (usize, usize)) {
        if !self.carts.contains_key(&coords) {
            return;
        }

        let mut cart = self.carts.get_mut(&coords).unwrap().clone();
        let new_coords = match cart.direction {
            Direction::Left => (coords.0 - 1, coords.1),
            Direction::Right => (coords.0 + 1, coords.1),
            Direction::Up => (coords.0, coords.1 - 1),
            Direction::Down => (coords.0, coords.1 + 1),
        };

        cart.direction = match self.map[new_coords.1][new_coords.0] {
            RailSegment::Horizontal | RailSegment::Vertical => cart.direction,
            RailSegment::CurveRight => match cart.direction {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
            },
            RailSegment::CurveLeft => match cart.direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Right,
            },
            RailSegment::Intersection => match cart.direction {
                Direction::Left => match cart.next_turn {
                    Turn::Left => Direction::Down,
                    Turn::Straight => Direction::Left,
                    Turn::Right => Direction::Up,
                },
                Direction::Right => match cart.next_turn {
                    Turn::Left => Direction::Up,
                    Turn::Straight => Direction::Right,
                    Turn::Right => Direction::Down,
                },
                Direction::Up => match cart.next_turn {
                    Turn::Left => Direction::Left,
                    Turn::Straight => Direction::Up,
                    Turn::Right => Direction::Right,
                },
                Direction::Down => match cart.next_turn {
                    Turn::Left => Direction::Right,
                    Turn::Straight => Direction::Down,
                    Turn::Right => Direction::Left,
                },
            },
            RailSegment::Empty => unreachable!(),
        };

        if self.map[new_coords.1][new_coords.0] == RailSegment::Intersection {
            cart.next_turn = match cart.next_turn {
                Turn::Left => Turn::Straight,
                Turn::Straight => Turn::Right,
                Turn::Right => Turn::Left,
            }
        };

        if self.carts.contains_key(&new_coords) {
            self.carts.remove(&coords);
            self.carts.remove(&new_coords);
            self.crushed_carts.insert(new_coords);
        } else {
            self.carts.remove(&coords);
            self.carts.insert(new_coords, cart);
        }
    }
}

impl Display for RailRoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if self.crushed_carts.contains(&(x, y)) {
                    write!(f, "X")?;
                } else if self.carts.contains_key(&(x, y)) {
                    write!(f, "{}", self.carts.get(&(x, y)).unwrap())?;
                } else {
                    write!(f, "{}", cell)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
