use crate::gce::board::types::{CastlingRights, Color, File, Move, MoveType, Piece, PieceType, Rank, Square};

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
                    board[file + (7 - rank) * 8] = Piece::from_string(sub_str) as u8;
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
        } as u8;
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

        let en_passant = if *split_fen.get(3).unwrap() == "-" {
            Square::None as u8
        } else {
            Square::from_string(split_fen.get(3).unwrap().to_string())
        };
        let fifty_move_rule = split_fen.get(4).unwrap().parse::<u8>().unwrap();
        let half_move_count = 2 * (split_fen.get(5).unwrap().parse::<u16>().unwrap() - 1) + if has_turn == Color::Black as u8 { 1 } else { 0 };

        NormalBoard {
            board,
            has_turn,
            castling_rights: castling_rights as u8,
            en_passant,
            fifty_move_rule,
            half_move_count
        }

    }

    pub fn do_move(&mut self, move_code: u16) {
        self.half_move_count += 1;
        self.fifty_move_rule += 1;

        if Move::type_of(move_code) == MoveType::Castling as u16 {
            
        }

        if Move::type_of(move_code) == MoveType::EnPassant as u16 {

        }
    }

    pub fn undo_move() {
        // TODO
    }

    fn compute_attacked_sqs() {

    }

    pub fn has_castle_right(&self, cr: u8) -> bool {
        cr & self.castling_rights != 0
    }

    pub fn side_to_move(&self) -> u8 {
        self.has_turn
    }

    pub fn get_sq_of(&self, piece: u8) -> Vec<u8> {
        let mut pieces: Vec<u8>;
        if Piece::type_of(piece) == PieceType::Pawn as u8 {
            pieces = Vec::with_capacity(8)
        } else {
            pieces = Vec::with_capacity(2);
        }
        let mut index: u8 = 0;
        for piece_on_board in self.board {
            if piece_on_board == piece {
                pieces.push(index);
            }
            index += 1;
        }
        pieces
    }

    pub fn get_ep_sq(&self) -> u8 {
        self.en_passant
    }

    pub fn set_to_at_index(&mut self, file: usize, rank: usize, piece: Piece) {
        self.board[file + rank * 8] = piece as u8;
    }

    pub fn set_to_at_file_and_rank(&mut self, file: File, rank: Rank, piece: Piece) {
        self.board[file as usize + (rank as usize * 8)] = piece as u8;
    }

    pub fn at_sq(&self, sq: u8) -> u8 {
        self.board[sq as usize]
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

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // board
        let mut empty_counter: u8;
        for rank in (0..8 as usize).rev() {
            empty_counter = 0;
            for file in 0..8 as usize {
                if self.board[rank * 8 + file] == 0 {
                    empty_counter += 1
                } else {
                    if empty_counter != 0 {
                        fen += &empty_counter.to_string();
                    }
                    fen += Piece::from_u8(self.board[rank * 8 + file]).to_string();
                    empty_counter = 0;
                }
            }
            if empty_counter != 0 {
                fen += &empty_counter.to_string();
            }
            if rank != 0 {
                fen += "/";
            }
        }

        // side to move
        fen += if self.has_turn == 0 { " w " } else { " b " };
        // castling rights
        fen += &CastlingRights::to_fen(self.castling_rights);
        // en passant
        if self.en_passant == Square::None as u8 {
            fen += " -"
        } else {
            fen += " ";
            fen += &Square::to_string(self.en_passant);
        };
        // fifty move rule
        fen += " ";
        fen += &self.fifty_move_rule.to_string();
        // move count
        fen += " ";
        fen += &(self.half_move_count / 2 + 1).to_string();
        fen
    }

}

