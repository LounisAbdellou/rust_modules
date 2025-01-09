use std::{thread, time::Duration, usize};

use ftkit::ARGS;

enum ParseError {
    InvalidWidth { arg: &'static str },
    InvalidHeight { arg: &'static str },
    InvalidPercentage { arg: &'static str },
    TooManyArguments,
    NotEnoughArguments,
}

#[derive(PartialEq, Copy, Clone)]
enum Cell {
    Dead,
    Alive,
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    virtual_cells: Vec<Cell>,
}

struct Point {
    x: isize,
    y: isize,
}

impl Cell {
    fn generate(percentage: u32) -> Self {
        let n = ftkit::random_number(0..=100) as u32;

        if n < percentage {
            return Cell::Alive;
        }

        return Cell::Dead;
    }

    fn is_alive(self) -> bool {
        return self == Self::Alive;
    }

    fn is_dead(self) -> bool {
        return self == Self::Dead;
    }
}

impl Board {
    fn new(width: usize, height: usize, percentage: u32) -> Self {
        let mut cells = Vec::new();
        for _ in 0..(width * height) {
            cells.push(Cell::generate(percentage));
        }

        let virtual_cells = cells.clone();
        let new_board = Self {
            width,
            height,
            cells,
            virtual_cells,
        };

        return new_board;
    }

    fn from_args() -> Result<Self, ParseError> {
        if ARGS.len() > 4 {
            return Err(ParseError::TooManyArguments);
        } else if ARGS.len() < 4 {
            return Err(ParseError::NotEnoughArguments);
        }

        let width = match ARGS[1].parse::<usize>() {
            Ok(width) => width,
            Err(..) => {
                return Err(ParseError::InvalidWidth { arg: &ARGS[1] });
            }
        };

        let height = match ARGS[2].parse::<usize>() {
            Ok(height) => height,
            Err(..) => {
                return Err(ParseError::InvalidHeight { arg: &ARGS[2] });
            }
        };

        let percentage = match ARGS[3].parse::<u32>() {
            Ok(percentage) => percentage,
            Err(..) => {
                return Err(ParseError::InvalidPercentage { arg: &ARGS[3] });
            }
        };

        return Ok(Self::new(width, height, percentage));
    }

    fn get_cell(&mut self, origin: &Point, mut dest: Point) -> Cell {
        let width = self.width as isize;
        let height = self.height as isize;

        if origin.x == width - 1 && dest.x > origin.x {
            dest.x = 0;
        } else if origin.x == 0 && dest.x < origin.x {
            dest.x = width - 1;
        }

        if origin.y == height - 1 && dest.y > origin.y {
            dest.y = 0;
        } else if origin.y == 0 && dest.y < origin.y {
            dest.y = height - 1;
        }

        return self.cells[(dest.x + (dest.y * width)) as usize];
    }

    fn apply_changes(&mut self) {
        for i in 0..(self.width * self.height) {
            self.cells[i] = self.virtual_cells[i];
        }
    }

    fn step(&mut self) {
        let width = self.width as isize;
        let height = self.height as isize;

        for y in 0..height {
            for x in 0..width {
                let mut neighbours = 0;
                let origin = Point { x, y };
                let cell_index = (x + (y * width)) as usize;

                neighbours += self.get_cell(&origin, Point { x: x + 1, y }).is_alive() as u32;
                neighbours += self.get_cell(&origin, Point { x: x - 1, y }).is_alive() as u32;
                neighbours += self.get_cell(&origin, Point { x, y: y + 1 }).is_alive() as u32;
                neighbours += self.get_cell(&origin, Point { x, y: y - 1 }).is_alive() as u32;
                neighbours += self
                    .get_cell(&origin, Point { x: x + 1, y: y - 1 })
                    .is_alive() as u32;
                neighbours += self
                    .get_cell(&origin, Point { x: x - 1, y: y + 1 })
                    .is_alive() as u32;
                neighbours += self
                    .get_cell(&origin, Point { x: x + 1, y: y + 1 })
                    .is_alive() as u32;
                neighbours += self
                    .get_cell(&origin, Point { x: x - 1, y: y - 1 })
                    .is_alive() as u32;

                match (self.cells[cell_index], neighbours) {
                    (Cell::Dead, 3) => self.virtual_cells[cell_index] = Cell::Alive,
                    (Cell::Alive, 4..=8) => self.virtual_cells[cell_index] = Cell::Dead,
                    (Cell::Alive, 0..=1) => self.virtual_cells[cell_index] = Cell::Dead,
                    (_, _) => (),
                };
            }
        }

        self.apply_changes();
    }

    fn print(&self, clear: bool) {
        if clear {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }

        for i in 0..(self.width * self.height) {
            if i % self.width == 0 {
                print!("\n");
            }

            if self.cells[i].is_alive() {
                print!("#");
            } else if self.cells[i].is_dead() {
                print!(".");
            }
        }
    }
}

fn main() {
    let mut board = match Board::from_args() {
        Ok(board) => board,
        Err(e) => match e {
            ParseError::TooManyArguments => {
                panic!("Error: too many arguments.")
            }
            ParseError::NotEnoughArguments => {
                panic!("Error: not enough arguments.")
            }
            ParseError::InvalidWidth { arg } => {
                panic!("Error: \"{}\" is not a valid width.", arg);
            }
            ParseError::InvalidHeight { arg } => {
                panic!("Error: \"{}\" is not a valid height.", arg);
            }
            ParseError::InvalidPercentage { arg } => {
                panic!("Error: \"{}\" is not a valid percentage.", arg);
            }
        },
    };

    board.print(true);

    loop {
        let duration = Duration::from_millis(250);
        thread::sleep(duration);

        board.step();
        board.print(true);
    }
}
