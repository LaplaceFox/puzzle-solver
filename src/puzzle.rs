use crate::puzzle::Verification::{Fail, Ok, Solution};
use core::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
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

#[derive(Clone, Copy, Debug)]
pub struct Board {
    cells: [Cell; 25],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [Cell::Unknown; 25],
        }
    }

    fn is_filled(&self) -> bool {
        !(self
            .cells
            .into_iter()
            .map(|x| matches!(x, Cell::Unknown))
            .fold(false, |x, y| x || y))
    }
}

#[test]
fn test_is_filled() {
    let mut full_board = Board {
        cells: [Cell::ValA; 25],
    };

    assert!(full_board.is_filled());

    // make an arbitrary cell unknown
    full_board.cells[13] = Cell::Unknown;

    assert!(!full_board.is_filled())
}

pub fn test_board() -> Board {
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

pub struct Puzzle {
    constraints: Vec<Constraint>,
    labels: ([Cell; 5], [Cell; 5], [Cell; 5], [Cell; 5]), //top, bot, left, right
    board: Board,
}

pub fn test_puzzle() -> Puzzle {
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
            format!("  {}  \n", bot.map(|x| x.to_string()).join(" ")),
        ]
        .join("");

        write!(f, "{}", output)
    }
}

impl Puzzle {
    fn verify(&self) -> Verification {
        for constraint in &self.constraints {
            if !((constraint.logic)(self.board)) {
                return Fail(constraint);
            }
        }

        if self.board.is_filled() {
            Solution(self.to_owned())
        } else {
            Ok
        }
    }
}

#[derive(Clone, Copy)]
pub enum LineType {
    Row,
    Col,
}

impl Display for LineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LineType::Row => "Row",
                LineType::Col => "Col",
            }
        )
    }
}

pub struct Constraint {
    name: String,
    logic: Box<dyn Fn(Board) -> bool>,
}

impl Constraint {
    fn line_check_on(ixs: [usize; 5]) -> Box<dyn Fn(Board) -> bool> {
        Box::new(move |board| {
            let mut counts = [0; 5];

            for ix in ixs {
                match board.cells[ix] {
                    Cell::ValA => counts[0] += 1,
                    Cell::ValB => counts[1] += 1,
                    Cell::ValC => counts[2] += 1,
                    Cell::ValD => counts[3] += 1,
                    Cell::Empty => counts[4] += 1,
                    _ => {}
                }
            }

            counts.into_iter().fold(true, |acc, x| acc && (x <= 1))
        })
    }

    fn line_check(lt: LineType, n: u8) -> Self {
        assert!(1 <= n && n <= 5);

        let i: usize = (5 * (n - 1)).into();

        let ixs = match lt {
            LineType::Row => [i, i + 1, i + 2, i + 3, i + 4],
            LineType::Col => [i, i + 5, i + 10, i + 15, i + 20],
        };

        let name = format!("Excess symbol in {} {}", lt, n);

        Constraint {
            name,
            logic: Constraint::line_check_on(ixs),
        }
    }
}

#[derive(Clone)]
pub enum Verification<'a> {
    Ok,                   // No obvious contradiction
    Fail(&'a Constraint), // At least one constraint not met
    Solution(&'a Puzzle), // Puzzle is solved
}
