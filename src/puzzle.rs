use crate::puzzle::Verification::{Fail, Solution, VerOk};
use core::fmt;
use std::{fmt::Display, str::FromStr};

use Cell::{Empty, Unknown, ValA, ValB, ValC, ValD};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    ValA,
    ValB,
    ValC,
    ValD,
    Empty,
    Unknown,
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(ValA),
            "B" => Ok(ValB),
            "C" => Ok(ValC),
            "D" => Ok(ValD),
            " " => Ok(Unknown),
            _ => Err(()),
        }
    }
}

#[test]
fn test_cell_from_str() {
    match Cell::from_str("A") {
        Ok(ValA) => (),
        _ => assert!(false),
    };

    match Cell::from_str("ABC") {
        Err(_) => (),
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

#[cfg(test)]
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

#[derive(Clone)]
pub struct Puzzle {
    labels: (Vec<Cell>, Vec<Cell>, Vec<Cell>, Vec<Cell>), //top, bot, left, right
    board: Board,
}

pub fn test_puzzle() -> Puzzle {
    Puzzle {
        labels: (
            vec![ValB, ValA, ValD, ValB, ValC],
            vec![ValA, ValB, Unknown, Unknown, ValB],
            vec![ValB, ValD, ValC, Unknown, Unknown],
            vec![ValC, ValA, Unknown, ValB, ValD],
        ),
        board: Board::new(5),
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

            output = [output, format!("{}│{}│{}\n", left[i], rowstr, right[i])].join("");
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

    fn get_first_seen(cells: &Vec<Cell>) -> Cell {
        for cell in cells {
            match cell {
                Empty => (), // Ignore, symbol is invisible
                letter => return letter.to_owned(),
            }
        }
        return Empty; // assume lack of column label is Unknown
    }

    fn get_line_first_seen(&self, ln: LineType, k: usize, rev: bool) -> Cell {
        let mut line = self.board.get_line(ln, k);

        if rev {
            line.reverse();
        }

        Puzzle::get_first_seen(&line)
    }

    pub fn verify(&self) -> Verification {
        // Duplicate checking
        for k in 0..5 {
            if !self.line_dupe_check(LineType::Row, k) {
                return Fail(FailReason::DuplicateSymbol(LineType::Row, k));
            }

            if !self.line_dupe_check(LineType::Col, k) {
                return Fail(FailReason::DuplicateSymbol(LineType::Col, k));
            }
        }

        // "Seen" checking
        let (top, bot, left, right) = &self.labels;

        for k in 0..5 {
            let seen_top = self.get_line_first_seen(LineType::Col, k, false);
            let seen_bot = self.get_line_first_seen(LineType::Col, k, true);
            let seen_left = self.get_line_first_seen(LineType::Row, k, false);
            let seen_right = self.get_line_first_seen(LineType::Row, k, true);

            if !(seen_top == top[k] || seen_top == Unknown) {
                return Fail(FailReason::ClueViolated(LineType::Col, k, false));
            }

            if !(seen_bot == bot[k] || seen_bot == Unknown) {
                return Fail(FailReason::ClueViolated(LineType::Col, k, true));
            }

            if !(seen_left == left[k] || seen_left == Unknown) {
                return Fail(FailReason::ClueViolated(LineType::Row, k, false));
            }

            if !(seen_right == right[k] || seen_right == Unknown) {
                return Fail(FailReason::ClueViolated(LineType::Row, k, true));
            }
        }

        if self.board.is_filled() {
            Solution(self.to_owned())
        } else {
            VerOk
        }
    }
}

#[test]
fn test_verify() {
    let mut puz = test_puzzle();

    match puz.verify() {
        VerOk => (),
        _ => assert!(false),
    }

    puz.board.cells[6] = ValC; // this is fine

    match puz.verify() {
        VerOk => (),
        _ => assert!(false),
    }

    puz.board.cells[1] = Empty; // violates seen rule

    match puz.verify() {
        Fail(reason) => assert_eq!(reason, FailReason::ClueViolated(LineType::Col, 1, false)),
        _ => assert!(false),
    }

    puz.board.cells[1] = Unknown;
    puz.board.cells[7] = ValC;

    match puz.verify() {
        Fail(reason) => assert_eq!(reason, FailReason::DuplicateSymbol(LineType::Row, 1)),
        _ => assert!(false),
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

#[test]
fn test_first_seen() {
    let mut cells = vec![Unknown; 5];

    assert_eq!(Unknown, Puzzle::get_first_seen(&cells));

    cells[4] = ValB;

    assert_eq!(Unknown, Puzzle::get_first_seen(&cells));

    cells[0] = Empty;
    cells[1] = ValA;

    assert_eq!(ValA, Puzzle::get_first_seen(&cells))
}

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FailReason {
    DuplicateSymbol(LineType, usize),
    ClueViolated(LineType, usize, bool),
}

impl Display for FailReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (ln, k) = match self {
            Self::DuplicateSymbol(ln, k) => (ln, k),
            Self::ClueViolated(ln, k, _) => (ln, k),
        };

        let desc = match self {
            Self::DuplicateSymbol(_, _) => "Duplicate symbol",
            Self::ClueViolated(LineType::Col, _, false) => "Top clue violated",
            Self::ClueViolated(LineType::Col, _, true) => "Bottom clue violated",
            Self::ClueViolated(LineType::Row, _, false) => "Left clue violated",
            Self::ClueViolated(LineType::Row, _, true) => "Right clue violated",
        };

        write!(f, "{} in {} {}", desc, ln.to_string(), k)
    }
}

#[derive(Clone)]
pub enum Verification {
    VerOk,            // No obvious contradiction
    Fail(FailReason), // At least one constraint not met
    Solution(Puzzle), // Puzzle is solved
}

impl Display for Verification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VerOk => "Ok".into(),
                Fail(reason) => format!("Failed: {}", reason),
                Solution(_) => "Solved".into(),
            }
        )
    }
}
