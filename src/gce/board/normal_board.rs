use std::borrow::BorrowMut;
use std::str::Split;
use crate::gce::board::types::{CastlingRights, Color, File, Piece, Rank, Square};
use crate::gce::board::types::Piece::*;

pub(crate) struct NormalBoard {

    board: [Piece; 64],
    has_turn: Color,
    castling_rights: CastlingRights,
    en_passant: Square
    

}

impl NormalBoard {

    pub fn new() -> NormalBoard {
        NormalBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn from_fen(fen: &str) -> NormalBoard {
        let split_fen: Vec<&str> = fen.split(" ").collect();

        let mut board: [Piece; 64] = [NoPiece; 64];
        let board_fen: Vec<&str> = split_fen.get(0).unwrap().split("/").collect();
        let mut sub_fen;
        let mut file: usize;
        for (rank, i) in board_fen.iter().enumerate() {
            sub_fen = i.split("");
            file = 0;
            for sub_str in sub_fen {
                if "rnbqkpRNBQKP".contains(sub_str){
                    board[file + (7 - rank) * 8] = Piece::from_str(sub_str);
                } else if "12345678".contains(sub_str) {
                    file += sub_str.parse::<usize>().unwrap();
                }
            }
        }

        let has_turn = match split_fen.get(1).unwrap() {
            &"w" => Color::White,
            &"b" => Color::Black,
            _ => panic!("Fehler im FEN!")
        };
        let mut castling_rights: CastlingRights = CastlingRights::NoCastling;
        if split_fen.get(2).unwrap() != &"-" {
            for right in split_fen.get(2).unwrap().split("").collect() {
                match right {
                    &"K" => castling_rights |= CastlingRights::WhiteOO,
                    &"Q" => castling_rights |= CastlingRights::WhiteOOO,
                    &"k" => castling_rights |= CastlingRights::BlackOO,
                    &"q" => castling_rights |= CastlingRights::BlackOOO,
                    _ => panic!("Fehler im FEN!")
                }
            }
        }
        if split_fen.get(3).unwrap() == "-" {
            let en_passant = Square::None;
        } else {

        }
        NormalBoard {
            board,
            has_turn,
            castling_rights,
            en_passant,
        }

    }

    pub fn set_to_at_index(&mut self, file: usize, rank: usize, piece: Piece) {
        self.board[file + rank * 8] = piece;
    }

    pub fn set_to_at_file_and_rank(&mut self, file: File, rank: Rank, piece: Piece) {
        self.board[(file + (rank * 8)) as usize] = piece;
    }

    pub fn at(&mut self, file: File, rank: Rank) -> &mut Piece {
        self.board[(file + (rank * 8)) as usize].borrow_mut()
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                s += self.board[file + rank * 8].to_string();
            }
            s += "\n";
        }
        s
    }

}

