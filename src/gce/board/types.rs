use std::ops::{Add, Mul};
use crate::gce::board::types::Piece::*;

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    NoPiece = 0,
    WPawn = 1, BPawn = 9,
    WKnight = 2, BKnight = 10,
    WBishop = 3, BBishop = 11,
    WRook = 4, BRook = 12,
    WQueen = 5, BQueen = 13,
    WKing = 6, BKing = 14
}

impl Piece {
    pub fn from_str(s: &str) -> Piece {
        match s {
            "p" => BPawn,
            "P" => WPawn,
            "n" => BKnight,
            "N" => WKnight,
            "b" => BBishop,
            "B" => WBishop,
            "r" => BRook,
            "R" => WRook,
            "q" => BQueen,
            "Q" => WQueen,
            "k" => BKing,
            "K" => WKing,
            &_ => NoPiece
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            NoPiece => " ",
            WPawn => "P",
            BPawn => "p",
            WKnight => "N",
            BKnight => "n",
            WBishop => "B",
            BBishop => "b",
            WRook => "R",
            BRook => "r",
            WQueen => "Q",
            BQueen => "q",
            WKing => "K",
            BKing => "k"
        }
    }
}

pub enum PieceType {
    NoPieceType = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen  = 5,
    King = 6
}

pub enum Color {
    White = 0,
    Black = 1
}

pub enum File {
    AFile,
    BFile,
    CFile,
    DFile,
    EFile,
    FFile,
    GFile,
    HFile
}

impl Add<i32> for File {
    type Output = i32;

    fn add(self, rhs: i32) -> Self::Output {
        self as i32 + rhs
    }
}

pub enum Rank {
    Rank1 = 0,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8
}

impl Mul<i32> for Rank {
    type Output = i32;

    fn mul(self, rhs: i32) -> Self::Output {
        self as i32 * rhs
    }
}
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
    None

}

pub enum CastlingRights {
    NoCastling = 0,
    WhiteOO    = 0b0001,
    WhiteOOO   = 0b0010,
    Black00    = 0b0100,
    BlackOOO   = 0b1000,
    KingSide   = 0b0101,
    QueenSide  = 0b1010,
    WhiteSide  = 0b0011,
    BlackSide  = 0b1100,
    AnyCastling= 0b1111
}
