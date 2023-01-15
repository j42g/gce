use crate::gce::board::types::CastlingRights::{BlackCastling, WhiteCastling};
use crate::gce::board::types::Piece::*;


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
    pub fn type_of(piece: u8) -> u8 {
        piece & 0b111
    }
    
    pub fn color_of(piece: u8) -> u8 {
        piece >> 3
    }
    
    pub fn from_string(c: char) -> Piece {
        match c.to_string().as_str() {
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
            &_ => panic!("Unknown piece: {}", c)

        }
    }

    pub fn from_u8(piece: u8) -> Piece {
        match piece {
            0 => NoPiece,
            1 => WPawn,
            2 => WKnight,
            3 => WBishop,
            4 => WRook,
            5 => WQueen,
            6 => WKing,
            9 => BPawn,
            10 => BKnight,
            11 => BBishop,
            12 => BRook,
            13 => BQueen,
            14 => BKing,
            _ => { println!("Invalid piece: {}", piece); NoPiece }
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

impl PieceType {
    pub fn from_string(piece: String) -> u16 {
        match piece.to_lowercase().as_str() {
            "p" => 1,
            "n" => 2,
            "b" => 3,
            "r" => 4,
            "q" => 5,
            "k" => 6,
            _ => panic!("Unknown Piecetype: {}", piece)
        }
    }

    pub fn to_string(piece: u16) -> String {
        match piece {
            1 => "p",
            2 => "n",
            3 => "b",
            4 => "r",
            5 => "q",
            6 => "k",
            _ => panic!("Unknown Piecetype: {}", piece)
        }.to_string()
    }
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

impl File {
    pub fn from_string(file: String) -> u8 {
        match file.to_lowercase().as_str() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => panic!("Invalid File: {}", file),
        }
    }

    pub fn to_string(file: u8) -> String {
        match file {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => panic!("Invalid File: {}", file),
        }.to_string()
    }

    pub fn file_of(sq: u8) -> u8 {
        sq & 7
    }
}

pub enum Rank {
    Rank1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8
}

impl Rank {
    pub fn from_string(rank: String) -> u8 {
        match rank.to_lowercase().as_str() {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            "5" => 4,
            "6" => 5,
            "7" => 6,
            "8" => 7,
            _ => panic!("Invalid Rank: {}", rank),
        }
    }

    pub fn to_string(rank: u8) -> String {
        if rank < 8 {
            (rank + 1).to_string()
        } else {
            panic!("Invalid rank: {}", rank)
        }

    }

    pub fn rank_of(sq: u8) -> u8 {
        sq >> 3
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

impl Square {
    pub fn from_string(sq: String) -> u8 { // sq soll sowas wie e3 sein, also "file""rank"
        let file = File::from_string(sq.chars().nth(0).unwrap().to_string());
        let rank = Rank::from_string(sq.chars().nth(1).unwrap().to_string());
        file + rank * 8
    }

    pub fn to_string(sq: u8) -> String {
        File::to_string(File::file_of(sq)) + &Rank::to_string(Rank::rank_of(sq))
    }
}

pub enum Move {
    MoveNone,
    MoveNull = 65
}

impl Move {
    pub fn from_string(move_string: String) -> u16 {
        let mut move_code: u16 = 0;
        if move_string.len() == 5 {
            move_code |= (PieceType::from_string(move_string.chars().nth(4).unwrap().to_string()) - 2) << 11;
            move_code |= MoveType::EnPassant as u16;
        }
        move_code |= Square::from_string(move_string[2..4].to_string()) as u16; // to square
        move_code | (Square::from_string(move_string[0..2].to_string()) as u16) << 5 // from square
    }

    pub fn to_string(move_code: u16) -> String {
        let mut move_string = Square::to_string(Move::from_sq_of(move_code));
        move_string += &Square::to_string(Move::to_sq_of(move_code));
        if (move_code >> 14) & 3 == 1 {
            move_string += &PieceType::to_string(((move_code >> 12) & 3) + 2);
        }
        move_string
    }

    pub fn from_sq_of(move_code: u16) -> u8 {
        ((move_code >> 6) & 0x3F) as u8
    }

    pub fn to_sq_of(move_code: u16) -> u8 {
        (move_code & 0x3F) as u8
    }
}

pub enum MoveType {
    Normal,
    Promotion = 1 << 14,
    EnPassant = 2 << 14,
    Castling  = 3 << 14
}

pub enum Direction {
    North = 8,
    East = 1,
    South = -8,
    West = -1,
    NorthEast = 9,
    SouthEast = -7,
    SouthWest = -9,
    NorthWest = 7,
    // Knight
    NorthNorthEast = 17,
    EastNorthEast = 10,
    EastSouthEast = -6,
    SouthSouthEast = -15,
    SouthSouthWest = -17,
    WestSouthWest = -10,
    WestNorthWest = 6,
    NorthNorthWest = 15,

}

pub enum CastlingRights {
    NoCastling = 0,
    WhiteOO    = 0b0001,
    WhiteOOO   = 0b0010,
    BlackOO    = 0b0100,
    BlackOOO   = 0b1000,
    KingSide   = 0b0101,
    QueenSide  = 0b1010,
    WhiteCastling = 0b0011,
    BlackCastling = 0b1100,
    AnyCastling= 0b1111
}

impl CastlingRights {
    pub fn cr_rights(side_to_move: u8, castling_rights: u8) -> u8 {
        (if side_to_move == 0 { WhiteCastling as u8} else { BlackCastling as u8 }) & castling_rights
    }
}
