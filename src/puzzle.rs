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
    pub fn new(n: usize) -> Self {
        Board {
            cells: vec![Unknown; n * n],
        }
    }

    pub fn get_line(&self, ln: LineType, k: usize) -> Vec<Cell> {
        // k is 0-indexed

        let n = 5; // TODO: Board size

        let (s, p) = match ln {
            LineType::Row => (k * n, 1),
            LineType::Col => (k, n),
        };

        { 0..n }.map(|x| self.cells[s + x * p]).collect()
    }

    pub fn is_filled(&self) -> bool {
        !(self
            .cells
            .iter()
            .map(|x| matches!(x, Unknown))
            .fold(false, |x, y| x || y))
    }
}

#[test]
fn test_get_line() {
    let board = test_board();

    let rowstr = board
        .get_line(LineType::Row, 2)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    assert_eq!(rowstr, "BCD  ");

    let colstr = board
        .get_line(LineType::Col, 1)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    assert_eq!(colstr, "A C  ")
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
    board.cells[10] = ValB;
    board.cells[11] = ValC;
    board.cells[12] = ValD;
    board.cells[23] = Empty;
    board.cells[24] = Empty;

    return board;
}

#[derive(Clone)]
pub struct Puzzle {
    labels: (Vec<Cell>, Vec<Cell>, Vec<Cell>, Vec<Cell>), //top, bot, left, right
    board: Board,
}

pub fn test_puzzle() -> Puzzle {
    Puzzle {
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
    // Checks there are no duplicate symbosl in row/col
    fn duplicate_check(cells: &Vec<Cell>) -> bool {
        let mut counts = [0; 5];

        for cell in cells {
            match cell {
                ValA => counts[0] += 1,
                ValB => counts[1] += 1,
                ValC => counts[2] += 1,
                ValD => counts[3] += 1,
                Empty => counts[4] += 1,
                _ => {}
            }
        }

        counts.into_iter().fold(true, |acc, x| acc && (x <= 1))
    }

    fn line_dupe_check(&self, ln: LineType, k: usize) -> bool {
        Puzzle::duplicate_check(&self.board.get_line(ln, k))
    }

    fn verify(self) -> Verification {
        // Duplicate checking
        for k in { 0..5 } {
            if !self.line_dupe_check(LineType::Row, k) {
                return Fail(format!("Duplicate symbol in Row {}", k));
            }

            if !self.line_dupe_check(LineType::Col, k) {
                return Fail(format!("Duplicate symbol in Col {}", k));
            }
        }

        // "Seen" checking

        if self.board.is_filled() {
            Solution(self)
        } else {
            Ok
        }
    }
}

#[test]
fn test_dup_check() {
    let mut cells = vec![Unknown; 5];

    assert!(Puzzle::duplicate_check(&cells));

    cells[0] = ValA;
    cells[1] = ValC;
    cells[2] = Empty;

    assert!(Puzzle::duplicate_check(&cells));

    cells[3] = ValC;

    //this should fail
    assert!(!Puzzle::duplicate_check(&cells));
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

#[derive(Clone)]
pub enum Verification {
    Ok,               // No obvious contradiction
    Fail(String),     // At least one constraint not met
    Solution(Puzzle), // Puzzle is solved
}

impl Verification {
    fn to_string(self) -> String {
        match self {
            Ok => "Ok".into(),
            Fail(msg) => format!("Failed: {}", msg),
            Solution(_) => "Solved".into(),
        }
    }
}
