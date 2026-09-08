
//     ################################
//     #                              #
//     #         Structs/Enum         #
//     #                              #
//     ################################

use crate::{Color::White, Rank::{Bishop, Rook}};


// Specifing what colors exist.
    // derive -> compiler can add prewritten implements.
#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
    Empty,
}
// Specifing what Rank a piece can be in.
#[derive(Copy, Clone, Debug)]
pub enum Rank {
    Demon,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    Empty,
}
// Struct for the pieces.
#[derive(Copy, Clone, Debug)]
pub struct Piece {
    color: Color,
    rank: Rank,
}
// Struct for the board.
pub struct Board {
    pub squares: [Piece; 64],
}


//     ################################
//     #                              #
//     #      Static Variables        #
//     #                              #
//     ################################


// Creates a null object = Empty, Empty
static NULLTOKEN: Piece = Piece { 
    color: Color::Empty, 
    rank: Rank::Empty,
};


//     ################################
//     #                              #
//     #           Methods            #
//     #                              #
//     ################################


// All chess logic.
impl Board {
    // Creates a board object with nothing on it.
    pub fn init_board() -> Board {
        // Creates the board with demon on one square and rest null.
        let mut gameboard = Board { 
            squares: [NULLTOKEN; 64],
        };
       
        //     ################################
        //     #        Init boardstate       #
        //     ################################ 
        
        // Places white rooks onto the board.
        gameboard.set_demon(0, White, Rook);
        gameboard.set_demon(7, White, Rook);
        
        // Places white bishop onto the board.
        gameboard.set_demon(2, White, Bishop);
        gameboard.set_demon(5, White, Bishop);
        
        
        
        
        
        
        
        
        return gameboard;
    }

    // Places a demon (Or any piece) onto the board.
    pub fn set_demon(&mut self, start: usize, color_piece: Color, rank_piece: Rank) {
         // Creates a demon object = White,Demon
        let demon = Piece { 
            color: color_piece,
            rank: rank_piece,
        }; 
        // Takes in a created Board object and sets the input square to the demon.      
        self.squares[start as usize] = demon;
    }

    // Moves a piece from one square (start) to another (dest), IF ALLOWED.
    pub fn move_piece(&mut self, start: usize, dest: usize) {
        // Matches what rank is on the starting square to fetch logic.
        match self.squares[start] {
            // Matches for move demon
            Piece {
                color,
                rank: Rank::Demon,
            } => self.move_demon(start, dest),
            // Matches for move with rook
            Piece {
                color: Color,
                rank: Rank::Rook,
            } => self.move_rook(start, dest),
            // Matches for move with bishop
            Piece {
                color: Color,
                rank: Rank::Bishop,
            } => self.move_bishop(start, dest),

            // Placeholder
            Piece {
                color: Color,
                rank: Rank,
            } => return
        }
    }

    //     ################################
    //     #         Move Logic           #
    //     ################################

    // Move logic of the Demon rank.
    fn move_demon(&mut self, start: usize, dest: usize) {
        self.squares[dest] = self.squares[start];
        self.squares[start] = NULLTOKEN;
    }
    // Move logic for the Rook rank.
    fn move_rook(&mut self, start: usize, dest: usize) {
        // Checks if move prompted is to the same offset on the x- or y-axis, else does nothing.
        if dest % 8 == start % 8 {
            self.move_demon(start, dest);
        } else if dest / 8 == start / 8 {
            self.move_demon(start, dest);
        } else {
            return;
        }
    }
    // Move logic for the Bishop.
    fn move_bishop(&mut self, start: usize, dest: usize) {
        // Checks if the difference + the original offset 
        if dest % 8 == (((((dest / 8) - (start / 8)) as f32).abs() as usize) + (start % 8)) {
            self.move_demon(start, dest);
        } 
        // Checks 
        else if dest % 8 == (((((dest / 8) - (start / 8)) as f32).abs() as usize) - (start % 8)){
            self.move_demon(start, dest);
        } else {
            return;
        }
    }
    // Move logic for the Knight
    fn move_knight(&mut self, start: usize, dest: usize){
        // Checks if the 

    }

}