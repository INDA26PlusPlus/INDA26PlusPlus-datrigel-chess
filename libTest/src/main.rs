use chess::*;

fn main() {
    let mut testboard = Board::init_board();
    testboard.set_demon(10); // Creates a demon on square 10.

    let testsquare3 = testboard.squares[10]; // Checks what piece is on the square.
    println!("Before move: {:?}", testsquare3);

    testboard.move_piece(10, 15); // Moves the piece on square 10 to square 15.


    let testsquare2 = testboard.squares[10]; // Checks what piece is on the square.
    println!("original square: {:?}", testsquare2);

    let testsquare1 = testboard.squares[15]; // Checks what piece is on the square.
    println!("final square: {:?}", testsquare1)


    

    
}