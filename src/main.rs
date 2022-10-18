use core::fmt;
use std::{collections::hash_map::IntoValues, fmt::format, process::Output};

#[derive(Copy, Clone)]
enum Cell {
    ValA,
    ValB,
    ValC,
    Empty,
    Unknown,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::ValA => "A",
                Cell::ValB => "B",
                Cell::ValC => "C",
                Cell::Empty => "*",
                Cell::Unknown => " ",
            }
        )
    }
}

struct Board {
    cells: [Cell; 25],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [Cell::Unknown; 25],
        }
    }
}

struct Puzzle {
    constraints: Vec<Constraint>,
    labels: ([Cell; 5], [Cell; 5], [Cell; 5], [Cell; 5]), //top, bot, left, right
    board: Board,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (top, bot, left, right) = self.labels;

        let mut output = format!("  {}  ", top.map(|x| x.to_string()).join(""));

        output = output + " ┌─────┐ ";

        let boardstr = self.board.cells.map(|x| x.to_string()).join("");

        for i in 0..5 {
            let rowstr = boardstr[5 * i..5 * (i + 1)].to_string();

            output = [output, format!("{}│{}│{}", left[0], rowstr, right[0])].join("");
        }

        output = output + " └─────┘ ";

        write!(f, "{}", output)
    }
}

type Constraint = fn(Board) -> bool;

fn main() {
    println!("Hello, world!");
}
