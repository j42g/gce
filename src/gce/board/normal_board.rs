use std::borrow::BorrowMut;
use std::str::Split;
use crate::gce::board::types;
use crate::gce::board::types::{CastlingRights, Color, File, Piece, Rank, Square};
use crate::gce::board::types::Piece::*;

pub(crate) struct NormalBoard {

    board: [u8; 64],
    has_turn: u8,
    castling_rights: u8,
    en_passant: u8,
    fifty_move_rule: u8, // moves since pawn push or capture
    half_move_count: u16
    

}

impl NormalBoard {

    pub fn new() -> NormalBoard {
        NormalBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn from_fen(fen: &str) -> NormalBoard {
        let split_fen: Vec<&str> = fen.split(" ").collect();
        let mut board: [u8; 64] = [0; 64];
        let board_fen: Vec<&str> = split_fen.get(0).unwrap().split("/").collect();
        let mut sub_fen;
        let mut file: usize;
        for (rank, i) in board_fen.iter().enumerate() {
            sub_fen = i.chars();
            file = 0;
            for sub_str in sub_fen {
                if "rnbqkpRNBQKP".contains(sub_str){
                    board[file + (7 - rank) * 8] = Piece::from_str(sub_str) as u8;
                    file += 1;
                } else if "12345678".contains(sub_str) {
                    file += sub_str as usize - 48;
                }
            }
        }

        let has_turn = match *split_fen.get(1).unwrap() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Fehler im FEN!")
        };
        let mut castling_rights: u8 = CastlingRights::NoCastling as u8;
        if *split_fen.get(2).unwrap() != "-" {
            let rights: Vec<_> = split_fen.get(2).unwrap().split("").collect();
            for right in rights {
                match right {
                    "K" => castling_rights |= CastlingRights::WhiteOO as u8,
                    "Q" => castling_rights |= CastlingRights::WhiteOOO as u8,
                    "k" => castling_rights |= CastlingRights::BlackOO as u8,
                    "q" => castling_rights |= CastlingRights::BlackOOO as u8,
                    "" => {/*Wenn man KQkq splittet kommt "", "K", ... , "" raus dewegen das hier*/}
                    _ => panic!("Fehler im FEN! Castlingright: \"{}\"", right)
                }
            }
        }
        let mut en_passant = Square::None as u8;
        if *split_fen.get(3).unwrap() != "-" {
            en_passant = Square::None as u8;
            // TODO
        }
        let fifty_move_rule = split_fen.get(4).unwrap().parse::<u8>().unwrap();
        let half_move_count = split_fen.get(5).unwrap().parse::<u16>().unwrap();

        NormalBoard {
            board,
            has_turn: has_turn as u8,
            castling_rights: castling_rights as u8,
            en_passant,
            fifty_move_rule,
            half_move_count
        }

    }

    pub fn set_to_at_index(&mut self, file: usize, rank: usize, piece: Piece) {
        self.board[file + rank * 8] = piece as u8;
    }

    pub fn set_to_at_file_and_rank(&mut self, file: File, rank: Rank, piece: Piece) {
        self.board[file as usize + (rank as usize * 8)] = piece as u8;
    }

    pub fn at(&mut self, file: File, rank: Rank) -> Piece {
        Piece::from_u8(self.board[file as usize + (rank as usize * 8)])
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                s += Piece::from_u8(self.board[file + rank * 8]).to_string();
            }
            s += "\n";
        }
        s
    }

}

