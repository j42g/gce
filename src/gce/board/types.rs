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
