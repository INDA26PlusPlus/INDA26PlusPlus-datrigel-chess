
//     ################################
//     #                              #
//     #         Structs/Enum         #
//     #                              #
//     ################################

use std::range;

use crate::{Color::{Black, White}, Rank::{Bishop, Rook}};


// Specifing what colors exist.
    // derive -> compiler can add prewritten implements.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
    Empty,
}
// Specifing what Rank a piece can be in.
#[derive(Copy, Clone, Debug, PartialEq)]
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
#[derive(Copy, Clone, Debug, PartialEq)]
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
            Piece {
                color: Color,
                rank: Rank::Knight,
            } => self.move_knight(start, dest),
            Piece {
                color: Color,
                rank: Rank::Queen,
            } => self.move_queen(start, dest), 
            Piece {
                color: Color,
                rank: Rank::King,
            } => self.move_king(start, dest),
            Piece {
                color: Color::Black,
                rank: Rank::Pawn,
            } => self.move_pawn(start, dest,Black),
            Piece {
                color: Color::White,
                rank: Rank::Pawn,
            } => self.move_pawn(start, dest, White), 
            Piece {
                color: Color,
                rank: Rank,
            } => return,
        }
    }

    //     ################################
    //     #      Core Move Logic         #
    //     ################################

    // Main method of moving pieces, Moves by copying to new square, setting old to nulltoken.
    fn move_demon(&mut self, start: usize, dest: usize) {
        self.squares[dest] = self.squares[start];
        self.squares[start] = NULLTOKEN;
    }
    fn check_square(&self, dest: usize) -> Piece {
        self.squares[dest]
    }

    //     ################################
    //     #     Piece Move Logic         #
    //     ################################

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
        // Checks if the difference + the original offset x-axis, 8 is used because the board is a 8x8 board.
        if (dest % 8) as f32 == (((((dest / 8) - (start / 8)) as f32).abs()) + ((start % 8) as f32)) {
            self.move_demon(start, dest);
        } 
        // Checks if the difference - the original offset x-axis
        else if (dest % 8) as f32 == (((((dest / 8) - (start / 8)) as f32).abs()) - ((start % 8) as f32)){
            self.move_demon(start, dest);
        } else {
            return;
        }
    }
    // Move logic for the Knight
    fn move_knight(&mut self, start: usize, dest: usize){
        let dest_offset = dest % 8;
        let start_offset = start % 8;
        // Checks if the knight is moving to a "standing L" OR "Laying L"
        if dest_offset == start_offset + 1 || dest_offset == start_offset - 1 {
            // checks distance is correct.
            if ((start as f32) - (dest as f32)).abs() != 2.0 {
                return;
            }
            self.move_demon(start, dest);
        } else if dest_offset == start_offset + 2 || dest_offset == start_offset - 2 {
            if ((start as f32) - (dest as f32)).abs() != 1.0 {
                return;
            }
            self.move_demon(start, dest);
        } else {
            return;
        }
    }
    // Move logic for the Queen.
    fn move_queen(&mut self, start: usize, dest: usize){
        // Moves like a bishop or knight.
        self.move_bishop(start,dest);
        self.move_rook(start,dest);
    }
    // Move logic for the king.
    fn move_king(&mut self, start: usize,dest: usize){
        // Checks if moves in 1 square difference.
        let distance = ((dest as f32) - (start as f32)).abs();
        let offset = (((dest % 8) as f32) - ((start % 8) as f32)).abs();
        if offset < 2.0 && distance < 2.0 {
            self.move_demon(start, dest);
        } else {
            return;
        }
    }
    // Move logic pawn.
    fn move_pawn(&mut self, start: usize, dest: usize,color: Color){
        // Need to set args to f32 in order to make calculations of negative numbers possible and .abs() usable
        let dest_i = dest as f32;
        let start_i = start as f32;
        // Checks what color the pawn is.
        if color == Color::White && (dest_i) - (start_i) / 8.0 == 1.0 {
            // Checks if pawn is trying to move forward or to take another piece
            if (dest_i) - (start_i) / 8.0 == 1.0 {
                // Checks if the path is clear to move to.
                if self.check_square(dest) == NULLTOKEN{
                    self.move_demon(start, dest);
                }
            } else if (dest_i - start_i) % 8.0 == 1.0{
                // Checks if a black piece can be moved to.
                if self.check_square(dest).color == Color::Black{
                    self.move_demon(start, dest);
                }
            }
        } else if color == Color::Black && (start_i) - (dest_i) / 8.0 == 1.0 {
            if (dest_i) - (start_i) / 8.0 == 1.0 {
                if self.check_square(dest) == NULLTOKEN{
                    self.move_demon(start, dest);
                }
            } else if (start_i - dest_i) % 8.0 == 1.0{
                if self.check_square(dest).color == Color::White{
                    self.move_demon(start, dest);
                }
            }
        } else {
            return;
        }
    }

}