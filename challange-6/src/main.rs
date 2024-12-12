use std::{
    fmt::{Debug, Formatter},
    fs,
};

fn main() {
    let text_input = fs::read_to_string("./input.txt").unwrap();

    let grid = text_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| Blocks::from_char(c))
                .collect::<Vec<Blocks>>()
        })
        .collect::<Vec<Vec<Blocks>>>();

    let mut start_x: i32 = 0;
    let mut start_y: i32 = 0;

    'outer: for x in 0..grid.len() {
        let line = &grid[x];
        for y in 0..line.len() {
            let block = &line[y];
            match block {
                Blocks::EmptyVisited { direction: _ } => {
                    start_x = x as i32;
                    start_y = y as i32;
                    break 'outer;
                }
                _ => {}
            }
        }
    }

    let mut map = Map(grid.clone(), start_x, start_y, Direction::Up);

    while map.move_guard() {}

    println!("{:?}", map);
    println!("{}", map.count_visited());

    let mut loop_count = 0;

    for x in 0..grid.len() {
        let sub_grid = &grid[x];
        for y in 0..sub_grid.len() {
            let current_block = &sub_grid[y];
            match current_block {
                Blocks::Blocked => continue,
                Blocks::EmptyVisited { .. } => continue,
                _ => {}
            }

            let mut clone = grid.clone();
            clone[x][y] = Blocks::Blocked;
            let mut map = Map(clone, start_x, start_y, Direction::Up);

            if map.check_loop() {
                loop_count += 1;
            }
        }
    }

    println!("loops: {}", loop_count);
}

struct Map(Vec<Vec<Blocks>>, i32, i32, Direction);

impl Map {
    fn is_guard_inside(&self, x: i32, y: i32) -> bool {
        x < self.0.len() as i32 && x >= 0 && y < self.0[0].len() as i32 && y >= 0
    }

    fn move_guard(&mut self) -> bool {
        let (x, y) = self.3.get_next_position(self.1, self.2);
        if !self.is_guard_inside(x, y) {
            return false;
        }
        let next_block = &mut self.0[x as usize][y as usize];
        match next_block {
            Blocks::Blocked => {
                self.3 = self.3.turn_right();
            }
            _ => {
                self.1 = x;
                self.2 = y;
                *next_block = Blocks::EmptyVisited { direction: self.3 };
            }
        };

        true
    }

    fn check_loop(&mut self) -> bool {
        let (x, y) = self.3.get_next_position(self.1, self.2);
        if !self.is_guard_inside(x, y) {
            return false;
        }
        let next_block = &mut self.0[x as usize][y as usize];
        match next_block {
            Blocks::Blocked => {
                self.3 = self.3.turn_right();
            }
            Blocks::EmptyVisited { direction } => {
                if *direction == self.3 {
                    return true;
                }
                self.1 = x;
                self.2 = y;
                *next_block = Blocks::EmptyVisited { direction: self.3 };
            }
            _ => {
                self.1 = x;
                self.2 = y;
                *next_block = Blocks::EmptyVisited { direction: self.3 };
            }
        };

        return self.check_loop();
    }

    fn count_visited(&self) -> i32 {
        self.0
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|b| match b {
                        Blocks::EmptyVisited { direction: _ } => true,
                        _ => false,
                    })
                    .count() as i32
            })
            .sum()
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let text = self
            .0
            .iter()
            .map(|line| line.iter().map(|b| b.to_char()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        writeln!(f, "{}", text)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn get_next_position(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Self::Up => (x - 1, y),
            Self::Right => (x, y + 1),
            Self::Down => (x + 1, y),
            Self::Left => (x, y - 1),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Blocks {
    Empty,
    EmptyVisited { direction: Direction },
    Blocked,
}

impl Blocks {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Blocked,
            '.' => Self::Empty,
            _ => Self::EmptyVisited {
                direction: Direction::Up,
            },
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::EmptyVisited { direction: _ } => 'X',
            Self::Blocked => '#',
        }
    }
}
