use chess::*;

fn main() {
    let mut testboard = Board::init_board();
    testboard.set_demon(10); // Creates a demon on square 10.
    let testsquare = testboard.squares[10]; // Need to implement indexmut method for Board.
    println!("{:?}", testsquare)
}