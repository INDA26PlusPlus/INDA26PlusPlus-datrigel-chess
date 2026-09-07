
//     ################################
//     #                              #
//     #         Structs/Enum         #
//     #                              #
//     ################################


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
        let gameboard = Board { 
            squares: [NULLTOKEN; 64],
        };
        return gameboard;
    }

    // Places a demon onto the board.
    pub fn set_demon(&mut self, start: usize) {
         // Creates a demon object = White,Demon
        let demon = Piece { 
            color: Color::White, 
            rank: Rank::Demon,
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
            // Placeholder
            Piece {
                color: Color,
                rank: Rank,
            } => return
        }
    }


    //     ################################
    //     #                              #
    //     #         Move Logic           #
    //     #                              #
    //     ################################


    // Move logic of the Demon rank.
    fn move_demon(&mut self, start: usize, dest: usize) {
        self.squares[dest as usize] = self.squares[start];
        self.squares[start] = NULLTOKEN;
    }

}