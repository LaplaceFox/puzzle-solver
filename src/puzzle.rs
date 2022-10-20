use core::fmt;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
        todo!()
    }
}

pub struct Constraint {
    name: String,
    logic: fn(Board) -> bool,
}

pub enum Verification {
    Ok,               // No obvious contradiction
    Fail(Constraint), // At least one constraint not met
    Solution(Puzzle), // Puzzle is solved
}
