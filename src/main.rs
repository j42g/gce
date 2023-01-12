extern crate core;
extern crate core;


use crate::gce::board::types::{File, Piece, Rank};
use crate::gce::board::normal_board::NormalBoard;

mod gce {
    pub(crate) mod board {
        pub(crate) mod normal_board;
        pub(crate) mod types;
        pub(crate) mod bitboard;
    }
}


fn main() {
    let a = NormalBoard::new();
    println!("{}", a.to_string())
}
