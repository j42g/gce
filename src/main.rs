extern crate core;


use crate::gce::board::types::{File, Move, Rank};
use crate::gce::board::normal_board::NormalBoard;
use crate::gce::moves::move_generation::{generate_all_moves, generate_bishop_moves, generate_king_moves, generate_pawn_moves, generate_queen_moves, generate_rook_moves};

mod gce {
    pub(crate) mod board {
        pub(crate) mod normal_board;
        pub(crate) mod types;
        pub(crate) mod bitboard;
    }
    pub(crate) mod moves{
        pub(crate) mod move_generation;
    }
}


fn main() { // rnbqkbnr/pppppppp/8/4b3/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1
    let a = NormalBoard::from_fen("8/8/8/2pP4/8/8/8/K6k w - c6 0 2");
    println!("{}", a.to_string());
    let mut movelist: [u16; 256] = [0; 256];
    let mut index: usize = 0;
    generate_all_moves(&a, a.side_to_move(), &mut movelist, &mut index);
    for i in 0..index {
        println!("{}", Move::to_string(movelist[i]));
    }
    println!("{}", a.to_fen());

}
