use crate::puzzle::Verification::{Fail, Ok, Solution};
use core::fmt;
use std::fmt::Display;

use Cell::{Empty, Unknown, ValA, ValB, ValC, ValD};

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    ValA,
    ValB,
    ValC,
    ValD,
    Empty,
    Unknown,
}

impl Cell {
    fn from_str(s: &str) -> Self {
        // TODO: implement FromStr?
        match s {
            "A" => ValA,
            "B" => ValB,
            "C" => ValC,
            "D" => ValD,
            _ => Unknown, // TODO: ideally, this would be an error
        }
    }
}

#[test]
fn test_cell_to_str() {
    match Cell::from_str("A") {
        ValA => (),
        _ => assert!(false),
    };

    match Cell::from_str("ABC") {
        Unknown => (),
        _ => assert!(false),
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ValA => "A",
                ValB => "B",
                ValC => "C",
                ValD => "D",
                Empty => "*",
                Unknown => " ",
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn new(n: usize) -> Self {
        Board {
            cells: vec![Unknown; n * n],
        }
    }

    fn is_filled(&self) -> bool {
        !(self
            .cells
            .iter()
            .map(|x| matches!(x, Unknown))
            .fold(false, |x, y| x || y))
    }
}

#[test]
fn test_is_filled() {
    let mut full_board = Board {
        cells: vec![ValA; 25],
    };

    assert!(full_board.is_filled());

    // make an arbitrary cell unknown
    full_board.cells[13] = Unknown;

    assert!(!full_board.is_filled())
}

pub fn test_board() -> Board {
    let mut board = Board::new(5);

    board.cells[1] = ValA;
    board.cells[3] = ValB;
    board.cells[5] = ValC;
    board.cells[7] = ValD;
    board.cells[9] = Empty;
    board.cells[23] = Empty;
    board.cells[24] = Empty;

    return board;
}

pub struct Puzzle {
    constraints: Vec<Constraint>,
    labels: (Vec<Cell>, Vec<Cell>, Vec<Cell>, Vec<Cell>), //top, bot, left, right
    board: Board,
}

pub fn test_puzzle() -> Puzzle {
    Puzzle {
        constraints: vec![],
        labels: (vec![ValA; 5], vec![ValB; 5], vec![ValC; 5], vec![ValD; 5]),
        board: test_board(),
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (top, bot, left, right) = &self.labels;

        let mut output = format!(
            "  {}  \n",
            top.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

        output = output + " ┌─────────┐ \n";

        let boardstr = self
            .board
            .cells
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        for i in 0..5 {
            let rowstr = boardstr[(10 * i)..(10 * i + 9)].to_string();

            output = [output, format!("{}│{}│{}\n", left[0], rowstr, right[0])].join("");
        }

        output = output + " └─────────┘ \n";

        output = [
            output,
            format!(
                "  {}  \n",
                bot.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        ]
        .join("");

        write!(f, "{}", output)
    }
}

impl Puzzle {
    fn verify(&self) -> Verification {
        for constraint in &self.constraints {
            if !((constraint.logic)(&self.board)) {
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
    logic: Box<dyn Fn(&Board) -> bool>,
}

impl Constraint {
    fn line_check_on(ixs: [usize; 5]) -> Box<dyn Fn(&Board) -> bool> {
        Box::new(move |board| {
            let mut counts = [0; 5];

            for ix in ixs {
                match board.cells[ix] {
                    ValA => counts[0] += 1,
                    ValB => counts[1] += 1,
                    ValC => counts[2] += 1,
                    ValD => counts[3] += 1,
                    Empty => counts[4] += 1,
                    _ => {}
                }
            }

            counts.into_iter().fold(true, |acc, x| acc && (x <= 1))
        })
    }

    // n is the row/column number, 1-indexed
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

#[test]
fn test_line_check() {
    let mut board = Board::new(5);

    let row2 = Constraint::line_check(LineType::Row, 3);

    assert!((*row2.logic)(&board)); // empty board, nothing wrong yet

    board.cells[10] = ValA;
    board.cells[11] = ValB;

    assert!((*row2.logic)(&board)); // no contradiction yet

    board.cells[12] = ValB; // duplicate value

    assert!(!((*row2.logic)(&board))); // this should fail
}

#[derive(Clone)]
pub enum Verification<'a> {
    Ok,                   // No obvious contradiction
    Fail(&'a Constraint), // At least one constraint not met
    Solution(&'a Puzzle), // Puzzle is solved
}

impl Verification<'_> {
    fn to_string(self) -> String {
        match self {
            Ok => "Ok".into(),
            Fail(c) => c.name.to_owned(),
            Solution(_) => "Solved".into(),
        }
    }
}
