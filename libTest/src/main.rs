use chess::*;

fn main() {
    let start: usize = 2;
    let dest: usize = 10;

    let mut testboard = Board::init_board();

    let testsquare3 = testboard.squares[start]; // Checks what piece is on the square.
    println!("Before move {}: {:?}", start, testsquare3);

    testboard.move_piece(start, dest); // Moves the piece on square 10 to square 15.


    let testsquare2 = testboard.squares[start]; // Checks what piece is on the square.
    println!("original square {}: {:?}", start, testsquare2);

    let testsquare1 = testboard.squares[dest]; // Checks what piece is on the square.
    println!("final square {}: {:?}", dest, testsquare1);

    println!("{} : {}", ((((dest / 8) - (start / 8))as f32).abs() as usize) + (start % 8), dest % 8)
}