mod puzzle;

use puzzle::test_puzzle;

fn main() {
    test_puzzle().verify();
    print!("{}", test_puzzle())
}
