mod puzzle;

use puzzle::{test_puzzle, Cell, Puzzle, Verification};
use Cell::{Empty, Unknown, ValA, ValB, ValC, ValD};
use Verification::{Fail, Solution, VerOk};

// Assumes that Puzzle is currently valid
fn solvestep(p: &Puzzle) -> Option<Verification> {
    let mut puz = p.clone(); // can avoid this with proper getter/setter

    for i in 0..25 {
        if puz.board.cells[i] == Unknown {
            // first unknown cell
            for mark in vec![ValA, ValB, ValC, ValD, Empty] {
                puz.board.cells[i] = mark;

                //println!("{}", puz);

                match puz.verify() {
                    VerOk => match solvestep(&puz) {
                        None => continue,
                        s => return s, // found a solution!
                    },
                    Fail(reason) => {
                        //println!("{}", reason);
                        continue;
                    }
                    Solution(sol) => return Some(Solution(sol)),
                }
            }

            return None;
        }
    }
    println!("Shouldn't be able to get here");
    None
}

fn solve(puz: Puzzle) -> Option<Verification> {
    match puz.verify() {
        VerOk => solvestep(&puz), // if valid, pass into brute-force loop
        Fail(_) => None,
        Solution(sol) => Some(Solution(sol)),
    }
}

fn main() {
    match solve(test_puzzle()) {
        Some(Solution(sol)) => println!("{}", sol),
        _ => println!("No solution"),
    }
}
