use std::borrow::BorrowMut;
use std::str::Split;
use crate::gce::board::types::{Color, File, Piece, Rank};
use crate::gce::board::types::Piece::*;

pub(crate) struct NormalBoard {

    board: [Piece; 64]
    

}

impl NormalBoard {

    pub fn new() -> NormalBoard {
        let board = [WRook, WKnight, WBishop, WQueen, WKing, WBishop, WKnight, WRook,
            WPawn, WPawn, WPawn, WPawn, WPawn, WPawn, WPawn, WPawn,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn,
            BRook, BKnight, BBishop, BQueen, BKing, BBishop, BKnight, BRook
        ];
        return NormalBoard {
            board
        }
    }

    pub fn from_fen(fen: &str) -> NormalBoard {
        let mut board: [Piece; 64] = [NoPiece; 64];


        let fen: Vec<&str> = fen.split("/").collect();
        let mut sub_fen;


        let mut file: usize;
        for (rank, i) in fen.iter().enumerate() {
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

        /*let mut sub_iter;
        for (rank, str) in fen.split("/").enumerate() {
            sub_iter = str.split("").enumerate().skip(0);
            for (file, sub_str) in sub_iter {
                if "rnbqkpRNBQKP".contains(sub_str){
                    board[file + (8 - rank) * 8] = Piece::from_str(sub_str);
                } else if "12345678".contains(sub_str) {
                    sub_iter = sub_iter.advance_by(1).unwrap();
                }
            }
        }*/
        let board = [WRook, WKnight, WBishop, WQueen, WKing, WBishop, WKnight, WRook,
            WPawn, WPawn, WPawn, WPawn, WPawn, WPawn, WPawn, WPawn,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece, NoPiece,
            BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BPawn,
            BRook, BKnight, BBishop, BQueen, BKing, BBishop, BKnight, BRook
        ];
        return NormalBoard {
            board
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

