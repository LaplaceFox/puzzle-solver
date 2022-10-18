use core::fmt;
use std::{collections::hash_map::IntoValues, fmt::format, process::Output};

#[derive(Clone, Copy)]
enum Cell {
    ValA,
    ValB,
    ValC,
    ValD,
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
                Cell::ValD => "D",
                Cell::Empty => "*",
                Cell::Unknown => " ",
            }
        )
    }
}

#[derive(Clone, Copy)]
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

fn test_board() -> Board {
    let mut board = Board::new();

    board.cells[1] = Cell::ValA;
    board.cells[3] = Cell::ValB;
    board.cells[5] = Cell::ValC;
    board.cells[7] = Cell::ValD;
    board.cells[9] = Cell::Empty;
    board.cells[23] = Cell::Empty;
    board.cells[24] = Cell::Empty;

    return board;
}

struct Puzzle {
    constraints: Vec<Constraint>,
    labels: ([Cell; 5], [Cell; 5], [Cell; 5], [Cell; 5]), //top, bot, left, right
    board: Board,
}

fn test_puzzle() -> Puzzle {
    Puzzle {
        constraints: vec![],
        labels: (
            [Cell::ValA; 5],
            [Cell::ValB; 5],
            [Cell::ValC; 5],
            [Cell::ValD; 5],
        ),
        board: test_board(),
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (top, bot, left, right) = self.labels;

        let mut output = format!("  {}  \n", top.map(|x| x.to_string()).join(" "));

        output = output + " ┌─────────┐ \n";

        let boardstr = self.board.cells.map(|x| x.to_string()).join(" ");

        for i in 0..5 {
            let rowstr = boardstr[(10 * i)..(10 * i + 9)].to_string();

            output = [output, format!("{}│{}│{}\n", left[0], rowstr, right[0])].join("");
        }

        output = output + " └─────────┘ \n";

        output = [
            output,
            format!("  {}  \n", top.map(|x| x.to_string()).join(" ")),
        ]
        .join("");

        write!(f, "{}", output)
    }
}

type Constraint = fn(Board) -> bool;

fn main() {
    print!("{}", test_puzzle())
}
