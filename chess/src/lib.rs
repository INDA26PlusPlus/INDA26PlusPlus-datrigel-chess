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
    Ki,
    Q,
    B,
    Kn,
    R,
    P,
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
impl Board {
    // Creates a board object with nothing on it.
    pub fn init_board() -> Board {
        // Creates a null object = Null,Null
        let nulltoken = Piece { 
            color: Color::Empty, 
            rank: Rank::Empty,
        };
        // Creates the board with demon on one square and rest null.
        let gameboard = Board { 
            squares: [nulltoken; 64],
        };
        return gameboard;
    }
}
// Places a demon onto the board.
impl Board {
    pub fn set_demon(&mut self, s: u32) {
         // Creates a demon object = White,Demon
        let demon = Piece { 
            color: Color::White, 
            rank: Rank::Demon,
        }; 
        // Takes in a created Board object and sets the input square to the demon.      
        self.squares[s as usize] = demon;
    }
}

impl Board {
    // Move a piece to any other square.
    fn move_piece(&self, s: u32, n: u32) {
        // match &self.squares[s] {
            
        // }
    }
}

