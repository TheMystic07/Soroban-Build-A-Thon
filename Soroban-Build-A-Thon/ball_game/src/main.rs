
use std::fmt;
use std::error::Error;
use std::fmt::{Formatter, Display};

//============== Part 2 Structs and Enums ==============

enum HorizDir {
    Left,
    Right,
}

enum VertDir {
    Up,
    Down,
}

struct Ball {
    x: i32,
    y: i32,
    horiz_dir: HorizDir,
    vert_dir: VertDir,
}

struct Frame {
    height: i32,
    width: i32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame {
                height: 62,
                width: 33,
            },
            ball: Ball {
                x: 44,
                y: 21,
                vert_dir: VertDir::Down,
                horiz_dir: HorizDir::Right,
            },
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x <= 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x >= frame.width {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y <= 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y >= frame.height {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "x")?;
        for _ in 0..64 { write!(fmt, "-")?; }
        writeln!(fmt)?;

        for y in 0..32 {
            for x in 0..64 {
                if self.ball.x == x as i32 && self.ball.y == y as i32 {
                    write!(fmt, "0")?;
                } else if x == 0 {
                    write!(fmt, "|")?;
                } else if x != 0 && y != 31 {
                    write!(fmt, " ")?;
                } else {
                    write!(fmt, "-")?;
                }
            }
            writeln!(fmt)?;
        }

        Ok(())
    }
}

fn main() {
    let mut game = Game::new();

    for _ in 0..10 {
        game.step();
        println!("{}", game);
    }
}
