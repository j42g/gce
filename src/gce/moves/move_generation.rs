use crate::gce::board::normal_board::NormalBoard;
use crate::gce::board::types::{CastlingRights, Direction::*, Direction, Move, MoveType, Piece, PieceType};

const KING_MOVES: [&[u8]; 64] = [&[8, 9, 1], &[9, 10, 2, 0, 8], &[10, 11, 3, 1, 9], &[11, 12, 4, 2, 10], &[12, 13, 5, 3, 11], &[13, 14, 6, 4, 12], &[14, 15, 7, 5, 13], &[15, 6, 14], &[16, 17, 9, 1, 0], &[17, 18, 10, 2, 1, 0, 8, 16], &[18, 19, 11, 3, 2, 1, 9, 17], &[19, 20, 12, 4, 3, 2, 10, 18], &[20, 21, 13, 5, 4, 3, 11, 19], &[21, 22, 14, 6, 5, 4, 12, 20], &[22, 23, 15, 7, 6, 5, 13, 21], &[23, 7, 6, 14, 22], &[24, 25, 17, 9, 8], &[25, 26, 18, 10, 9, 8, 16, 24], &[26, 27, 19, 11, 10, 9, 17, 25], &[27, 28, 20, 12, 11, 10, 18, 26], &[28, 29, 21, 13, 12, 11, 19, 27], &[29, 30, 22, 14, 13, 12, 20, 28], &[30, 31, 23, 15, 14, 13, 21, 29], &[31, 15, 14, 22, 30], &[32, 33, 25, 17, 16], &[33, 34, 26, 18, 17, 16, 24, 32], &[34, 35, 27, 19, 18, 17, 25, 33], &[35, 36, 28, 20, 19, 18, 26, 34], &[36, 37, 29, 21, 20, 19, 27, 35], &[37, 38, 30, 22, 21, 20, 28, 36], &[38, 39, 31, 23, 22, 21, 29, 37], &[39, 23, 22, 30, 38], &[40, 41, 33, 25, 24], &[41, 42, 34, 26, 25, 24, 32, 40], &[42, 43, 35, 27, 26, 25, 33, 41], &[43, 44, 36, 28, 27, 26, 34, 42], &[44, 45, 37, 29, 28, 27, 35, 43], &[45, 46, 38, 30, 29, 28, 36, 44], &[46, 47, 39, 31, 30, 29, 37, 45], &[47, 31, 30, 38, 46], &[48, 49, 41, 33, 32], &[49, 50, 42, 34, 33, 32, 40, 48], &[50, 51, 43, 35, 34, 33, 41, 49], &[51, 52, 44, 36, 35, 34, 42, 50], &[52, 53, 45, 37, 36, 35, 43, 51], &[53, 54, 46, 38, 37, 36, 44, 52], &[54, 55, 47, 39, 38, 37, 45, 53], &[55, 39, 38, 46, 54], &[56, 57, 49, 41, 40], &[57, 58, 50, 42, 41, 40, 48, 56], &[58, 59, 51, 43, 42, 41, 49, 57], &[59, 60, 52, 44, 43, 42, 50, 58], &[60, 61, 53, 45, 44, 43, 51, 59], &[61, 62, 54, 46, 45, 44, 52, 60], &[62, 63, 55, 47, 46, 45, 53, 61], &[63, 47, 46, 54, 62], &[57, 49, 48], &[58, 50, 49, 48, 56], &[59, 51, 50, 49, 57], &[60, 52, 51, 50, 58], &[61, 53, 52, 51, 59], &[62, 54, 53, 52, 60], &[63, 55, 54, 53, 61], &[55, 54, 62]];

pub(crate) fn generate_king_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    let king_orig = *board.get_sq_of((PieceType::King as u8) | side_to_move << 3).first().unwrap() as u8;
    let queenside_rook = if side_to_move == 0 { 0 } else { 56 };
    let kingside_rook = if side_to_move == 0 { 7 } else { 63 };

    // normal moves
    for king_dest in KING_MOVES[king_orig as usize] {
        if move_dest_empty_or_capture(board, side_to_move, *king_dest) {
            move_list[*index] = Move::make_move(king_orig, *king_dest);
            *index += 1;
        }
    }

    // castling
    if board.has_castle_right(CastlingRights::cr_rights(side_to_move, CastlingRights::AnyCastling as u8)) {
        if board.has_castle_right(CastlingRights::cr_rights(side_to_move, CastlingRights::KingSide as u8)) { // Kingside
            move_list[*index] = MoveType::Castling as u16 | Move::make_move(king_orig, kingside_rook);
            *index += 1;
        }
        if board.has_castle_right(CastlingRights::cr_rights(side_to_move, CastlingRights::QueenSide as u8)) { // Queenside
            move_list[*index] = MoveType::Castling as u16 | Move::make_move(king_orig, queenside_rook);
            *index += 1;
        }
    }
}

pub(crate) fn generate_pawn_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    let pawn_pos = board.get_sq_of((PieceType::Pawn as u8) | side_to_move << 3);
    let direction = if side_to_move == 0 { North as u8 } else { South as u8 }; // directions pawn move for side
    let prom_upper_bound = if side_to_move == 0 { 56 } else { 16 }; // promotion rank
    let prom_lower_bound = if side_to_move == 0 { 47 } else { 7 }; // promotion rank
    let two_upper_bound = if side_to_move == 0 { 16 } else { 56 }; // 2x move rank
    let two_lower_bound = if side_to_move == 0 { 7 } else { 47 }; // 2x move rank
    let mut pawn_dest;
    for pawn_orig in pawn_pos {
        if prom_lower_bound < pawn_orig && pawn_orig < prom_upper_bound { // promotions
            pawn_dest = pawn_orig + direction;
            if board.at_sq(pawn_dest as u8) as u16 == Piece::NoPiece as u16 { // move forward
                add_promotions(pawn_orig, pawn_dest, move_list, index);
            }
            if pawn_orig & 0x7 != 7 { // right capture for white / left for black
                pawn_dest = pawn_orig + direction + 1;
                if Piece::color_of(board.at_sq(pawn_dest as u8)) != side_to_move {
                    add_promotions(pawn_orig, pawn_dest, move_list, index);
                }
            }
            if pawn_orig & 0x7 != 0 { // left capture for white / right for black
                pawn_dest = pawn_orig + direction - 1;
                if Piece::color_of(board.at_sq(pawn_dest as u8)) != side_to_move {
                    add_promotions(pawn_orig, pawn_dest, move_list, index);
                }
            }
        } else { // not about to promote
            pawn_dest = pawn_orig + direction;
            if board.at_sq(pawn_dest) == Piece::NoPiece as u8 { // can move 1 forward
                move_list[*index] = ((pawn_orig as u16) << 6) | (pawn_dest as u16);
                *index += 1;
                if two_lower_bound < pawn_orig && pawn_orig < two_upper_bound { // are we on first pawn line
                    pawn_dest += direction;
                    if board.at_sq(pawn_dest) == Piece::NoPiece as u8 {
                        move_list[*index] = ((pawn_orig as u16) << 6) | (pawn_dest as u16);
                        *index += 1;
                    }
                }
            }
        }
    }
    let ep_sq = board.get_ep_sq();
    if ep_sq != Piece::NoPiece as u8 { // pawn_dest acts as a origin here
        if ep_sq & 0x7 != 7 { // left capture for white / right for black
            pawn_dest = ep_sq - direction + 1;
            if Piece::type_of(board.at_sq(pawn_dest)) == PieceType::Pawn as u8 {
                add_en_passent(pawn_dest, ep_sq, move_list, index);
            }
        } else if ep_sq & 0x7 != 0 { // only right capture for white / left for black
            pawn_dest = ep_sq - direction - 1;
            if Piece::type_of(board.at_sq(pawn_dest)) == PieceType::Pawn as u8 {
                add_en_passent(pawn_dest, ep_sq, move_list, index);
            }
        }
    }
}

fn add_promotions(pawn_orig: u8, pawn_dest: u8, move_list: &mut [u16; 256], index: &mut usize) {
    for piece in 0..=3 as u16 {
        move_list[*index] = (MoveType::Promotion as u16) | (piece << 12) | ((pawn_orig as u16) << 6) | (pawn_dest as u16);
        *index += 1;
    }
}

fn add_en_passent(pawn_orig: u8, pawn_dest: u8, move_list: &mut [u16; 256], index: &mut usize) {
    move_list[*index] = (MoveType::EnPassant as u16) | ((pawn_orig as u16) << 6) | (pawn_dest as u16);
    *index += 1;
}

const KNIGHT_MOVES: [&[u8]; 64] = [&[17, 10], &[18, 11, 16], &[19, 12, 8, 17], &[20, 13, 9, 18], &[21, 14, 10, 19], &[22, 15, 11, 20], &[23, 12, 21], &[13, 22], &[25, 18, 2], &[26, 19, 3, 24], &[27, 20, 4, 0, 16, 25], &[28, 21, 5, 1, 17, 26], &[29, 22, 6, 2, 18, 27], &[30, 23, 7, 3, 19, 28], &[31, 4, 20, 29], &[5, 21, 30], &[33, 26, 10, 1], &[34, 27, 11, 2, 0, 32], &[35, 28, 12, 3, 1, 8, 24, 33], &[36, 29, 13, 4, 2, 9, 25, 34], &[37, 30, 14, 5, 3, 10, 26, 35], &[38, 31, 15, 6, 4, 11, 27, 36], &[39, 7, 5, 12, 28, 37], &[6, 13, 29, 38], &[41, 34, 18, 9], &[42, 35, 19, 10, 8, 40], &[43, 36, 20, 11, 9, 16, 32, 41], &[44, 37, 21, 12, 10, 17, 33, 42], &[45, 38, 22, 13, 11, 18, 34, 43], &[46, 39, 23, 14, 12, 19, 35, 44], &[47, 15, 13, 20, 36, 45], &[14, 21, 37, 46], &[49, 42, 26, 17], &[50, 43, 27, 18, 16, 48], &[51, 44, 28, 19, 17, 24, 40, 49], &[52, 45, 29, 20, 18, 25, 41, 50], &[53, 46, 30, 21, 19, 26, 42, 51], &[54, 47, 31, 22, 20, 27, 43, 52], &[55, 23, 21, 28, 44, 53], &[22, 29, 45, 54], &[57, 50, 34, 25], &[58, 51, 35, 26, 24, 56], &[59, 52, 36, 27, 25, 32, 48, 57], &[60, 53, 37, 28, 26, 33, 49, 58], &[61, 54, 38, 29, 27, 34, 50, 59], &[62, 55, 39, 30, 28, 35, 51, 60], &[63, 31, 29, 36, 52, 61], &[30, 37, 53, 62], &[58, 42, 33], &[59, 43, 34, 32], &[60, 44, 35, 33, 40, 56], &[61, 45, 36, 34, 41, 57], &[62, 46, 37, 35, 42, 58], &[63, 47, 38, 36, 43, 59], &[39, 37, 44, 60], &[38, 45, 61], &[50, 41], &[51, 42, 40], &[52, 43, 41, 48], &[53, 44, 42, 49], &[54, 45, 43, 50], &[55, 46, 44, 51], &[47, 45, 52], &[46, 53]];

pub(crate) fn generate_knight_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    let knight_pos = board.get_sq_of((PieceType::Knight as u8) | side_to_move << 3);
    for knight_orig in knight_pos {
        for knight_dest in KNIGHT_MOVES[knight_orig as usize] {
            if move_dest_empty_or_capture(board, side_to_move, *knight_dest) {
                move_list[*index] = Move::make_move(knight_orig, *knight_dest as u8);
                *index += 1;
            }
        }
    }
}

pub(crate) fn generate_rook_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    let directions = [North as i8, East as i8, South as i8, West as i8];
    let rook_pos = board.get_sq_of((PieceType::Rook as u8) | side_to_move << 3);
    let mut curr_pos: i8;
    for rook_orig in rook_pos {
        for direction in directions {
            curr_pos = rook_orig as i8;
            loop {
                curr_pos += direction;
                if 0 > curr_pos || curr_pos > 63 {
                    break;
                } else if (direction == North as i8 && curr_pos < 8)
                    || (direction == West as i8 && curr_pos & 0x7 == 7)
                    || (direction == South as i8 && curr_pos > 55)
                    || (direction == East as i8 && curr_pos & 0x7 == 0) { // edge of board
                    break;
                } else if board.at_sq(curr_pos as u8) == Piece::NoPiece as u8 {
                    move_list[*index] = Move::make_move(rook_orig, curr_pos as u8);
                    *index += 1;
                } else {
                    if Piece::color_of(board.at_sq(curr_pos as u8)) != side_to_move {
                        move_list[*index] = Move::make_move(rook_orig, curr_pos as u8);
                        *index += 1;
                    }
                    break;
                }
            }
        }
    }
}

pub(crate) fn generate_bishop_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    let directions = [NorthEast as i8, SouthEast as i8, SouthWest as i8, NorthWest as i8];
    let bishop_pos = board.get_sq_of((PieceType::Bishop as u8) | side_to_move << 3);
    let mut curr_pos;
    for bishop_orig in bishop_pos {
        for direction in directions {
            curr_pos = bishop_orig as i8;
            loop {
                curr_pos += direction;
                if 0 > curr_pos || curr_pos > 63 {
                    break;
                } else if (direction == North as i8 && curr_pos < 8)
                    || (direction == West as i8 && curr_pos & 0x7 == 7)
                    || (direction == South as i8 && curr_pos > 55)
                    || (direction == East as i8 && curr_pos & 0x7 == 0) { // edge of board
                    break;
                } else if board.at_sq(curr_pos as u8) == Piece::NoPiece as u8 {
                    move_list[*index] = Move::make_move(bishop_orig, curr_pos as u8);
                    *index += 1;
                } else {
                    if Piece::color_of(board.at_sq(curr_pos as u8)) != side_to_move {
                        move_list[*index] = Move::make_move(bishop_orig, curr_pos as u8);
                        *index += 1;
                    }
                    break;
                }
            }
        }
    }
}

pub(crate) fn generate_queen_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    let directions = [North as i8, East as i8, South as i8, West as i8, NorthEast as i8, SouthEast as i8, SouthWest as i8, NorthWest as i8];
    let queen_pos = board.get_sq_of((PieceType::Queen as u8) | side_to_move << 3);
    let mut curr_pos;
    for queen_orig in queen_pos {
        for direction in directions {
            curr_pos = queen_orig as i8;
            loop {
                curr_pos += direction;
                if 0 > curr_pos || curr_pos > 63 {
                    break;
                } else if (direction == North as i8 && curr_pos < 8)
                    || (direction == West as i8 && curr_pos & 0x7 == 7)
                    || (direction == South as i8 && curr_pos > 55)
                    || (direction == East as i8 && curr_pos & 0x7 == 0)
                    || (direction == NorthEast as i8 && curr_pos & 0x7 == 0)
                    || (direction == SouthEast as i8 && curr_pos & 0x7 == 0)
                    || (direction == SouthWest as i8 && curr_pos & 0x7 == 7)
                    || (direction == NorthWest as i8 && curr_pos & 0x7 == 7) { // edge of board
                    break;
                } else if board.at_sq(curr_pos as u8) == Piece::NoPiece as u8 {
                    move_list[*index] = Move::make_move(queen_orig, curr_pos as u8);
                    *index += 1;
                } else {
                    if Piece::color_of(board.at_sq(curr_pos as u8)) != side_to_move {
                        move_list[*index] = Move::make_move(queen_orig, curr_pos as u8);
                        *index += 1;
                    }
                    break;
                }
            }
        }
    }
}

pub(crate) fn generate_all_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    generate_pawn_moves(board, side_to_move, move_list, index);
    generate_knight_moves(board, side_to_move, move_list, index);
    generate_bishop_moves(board, side_to_move, move_list, index);
    generate_rook_moves(board, side_to_move, move_list, index);
    generate_queen_moves(board, side_to_move, move_list, index);
    generate_king_moves(board, side_to_move, move_list, index);
}

fn move_dest_empty_or_capture(board: &NormalBoard, side_to_move: u8, piece_dest: u8) -> bool {
    let piece_at_dest = board.at_sq(piece_dest);
    (piece_at_dest == Piece::NoPiece as u8) || (Piece::color_of(piece_at_dest) != side_to_move)
}