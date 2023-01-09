extern crate core;

use crate::gce::board::types::{File, Piece, Rank};
use crate::gce::board::normal_board::NormalBoard;

mod gce {
    pub(crate) mod board {
        pub(crate) mod normal_board;
        pub(crate) mod types;
    }
}


fn main() {
    let a = NormalBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    println!("{}", a.to_string())
}
