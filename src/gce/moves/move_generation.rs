use crate::gce::board::normal_board::NormalBoard;
use crate::gce::board::types::{Direction::*, Direction, MoveType, Piece, PieceType};

pub(crate) fn generate_king_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    let kingpos = *board.get_sq_of((PieceType::King as u8) | side_to_move << 3).first().unwrap() as i16;
    let north = kingpos < 56;
    let east = kingpos & 0x7 != 7;
    let south = kingpos > 8;
    let west = kingpos & 0x7 != 0;
    let mut king_dest;
    if north {
        king_dest = (kingpos + North as i16) as u16;
        if move_dest_empty_or_capture(board, side_to_move, king_dest) {
            move_list[*index] = (((kingpos as u16) << 6) | king_dest) as u16;
            *index += 1;
        }
    }
    if north && east {
        king_dest = (kingpos + NorthEast as i16) as u16;
        if move_dest_empty_or_capture(board, side_to_move, king_dest) {
            move_list[*index] = (((kingpos as u16) << 6) | king_dest) as u16;
            *index += 1;
        }
    }
    if east {
        king_dest = (kingpos + East as i16) as u16;
        if move_dest_empty_or_capture(board, side_to_move, king_dest) {
            move_list[*index] = (((kingpos as u16) << 6) | king_dest) as u16;
            *index += 1;
        }
    }
    if south && east {
        king_dest = (kingpos + SouthEast as i16) as u16;
        if move_dest_empty_or_capture(board, side_to_move, king_dest) {
            move_list[*index] = (((kingpos as u16) << 6) | king_dest) as u16;
            *index += 1;
        }
    }
    if south {
        king_dest = (kingpos + South as i16) as u16;
        if move_dest_empty_or_capture(board, side_to_move, king_dest) {
            move_list[*index] = (((kingpos as u16) << 6) | king_dest) as u16;
            *index += 1;
        }
    }
    if south && west {
        king_dest = (kingpos + SouthWest as i16) as u16;
        if move_dest_empty_or_capture(board, side_to_move, king_dest) {
            move_list[*index] = (((kingpos as u16) << 6) | king_dest) as u16;
            *index += 1;
        }
    }
    if west {
        king_dest = (kingpos + West as i16) as u16;
        if move_dest_empty_or_capture(board, side_to_move, king_dest) {
            move_list[*index] = (((kingpos as u16) << 6) | king_dest) as u16;
            *index += 1;
        }
    }
    if north && west {
        king_dest = (kingpos + NorthWest as i16) as u16;
        if move_dest_empty_or_capture(board, side_to_move, king_dest) {
            move_list[*index] = (((kingpos as u16) << 6) | king_dest) as u16;
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

pub(crate) fn generate_knight_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    let knight_moves: [Vec<u16>; 64] = [vec![17, 10], vec![18, 11, 16], vec![19, 12, 8, 17], vec![20, 13, 9, 18], vec![21, 14, 10, 19], vec![22, 15, 11, 20], vec![23, 12, 21], vec![13, 22], vec![25, 18, 2], vec![26, 19, 3, 24], vec![27, 20, 4, 0, 16, 25], vec![28, 21, 5, 1, 17, 26], vec![29, 22, 6, 2, 18, 27], vec![30, 23, 7, 3, 19, 28], vec![31, 4, 20, 29], vec![5, 21, 30], vec![33, 26, 10, 1], vec![34, 27, 11, 2, 0, 32], vec![35, 28, 12, 3, 1, 8, 24, 33], vec![36, 29, 13, 4, 2, 9, 25, 34], vec![37, 30, 14, 5, 3, 10, 26, 35], vec![38, 31, 15, 6, 4, 11, 27, 36], vec![39, 7, 5, 12, 28, 37], vec![6, 13, 29, 38], vec![41, 34, 18, 9], vec![42, 35, 19, 10, 8, 40], vec![43, 36, 20, 11, 9, 16, 32, 41], vec![44, 37, 21, 12, 10, 17, 33, 42], vec![45, 38, 22, 13, 11, 18, 34, 43], vec![46, 39, 23, 14, 12, 19, 35, 44], vec![47, 15, 13, 20, 36, 45], vec![14, 21, 37, 46], vec![49, 42, 26, 17], vec![50, 43, 27, 18, 16, 48], vec![51, 44, 28, 19, 17, 24, 40, 49], vec![52, 45, 29, 20, 18, 25, 41, 50], vec![53, 46, 30, 21, 19, 26, 42, 51], vec![54, 47, 31, 22, 20, 27, 43, 52], vec![55, 23, 21, 28, 44, 53], vec![22, 29, 45, 54], vec![57, 50, 34, 25], vec![58, 51, 35, 26, 24, 56], vec![59, 52, 36, 27, 25, 32, 48, 57], vec![60, 53, 37, 28, 26, 33, 49, 58], vec![61, 54, 38, 29, 27, 34, 50, 59], vec![62, 55, 39, 30, 28, 35, 51, 60], vec![63, 31, 29, 36, 52, 61], vec![30, 37, 53, 62], vec![58, 42, 33], vec![59, 43, 34, 32], vec![60, 44, 35, 33, 40, 56], vec![61, 45, 36, 34, 41, 57], vec![62, 46, 37, 35, 42, 58], vec![63, 47, 38, 36, 43, 59], vec![39, 37, 44, 60], vec![38, 45, 61], vec![50, 41], vec![51, 42, 40], vec![52, 43, 41, 48], vec![53, 44, 42, 49], vec![54, 45, 43, 50], vec![55, 46, 44, 51], vec![47, 45, 52], vec![46, 53]];
    let knight_pos = board.get_sq_of((PieceType::Knight as u8) | side_to_move << 3);
    for knight_orig in knight_pos {
        for knight_dest in &knight_moves[knight_orig as usize] {
            if move_dest_empty_or_capture(board, side_to_move, *knight_dest) {
                move_list[*index] = ((knight_orig as u16) << 6) | knight_dest;
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
                    move_list[*index] = ((rook_orig as u16) << 6) | curr_pos as u16;
                    *index += 1;
                } else {
                    if Piece::color_of(board.at_sq(curr_pos as u8)) != side_to_move {
                        move_list[*index] = ((rook_orig as u16) << 6) | curr_pos as u16;
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
                    move_list[*index] = ((bishop_orig as u16) << 6) | curr_pos as u16;
                    *index += 1;
                } else {
                    if Piece::color_of(board.at_sq(curr_pos as u8)) != side_to_move {
                        move_list[*index] = ((bishop_orig as u16) << 6) | curr_pos as u16;
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
                    move_list[*index] = ((queen_orig as u16) << 6) | curr_pos as u16;
                    *index += 1;
                } else {
                    if Piece::color_of(board.at_sq(curr_pos as u8)) != side_to_move {
                        move_list[*index] = ((queen_orig as u16) << 6) | curr_pos as u16;
                        *index += 1;
                    }
                    break;
                }
            }
        }
    }
}

pub(crate) fn generate_all_moves(board: &NormalBoard, side_to_move: u8, move_list: &mut [u16; 256], index: &mut usize) {
    generate_knight_moves(board, side_to_move, move_list, index);
    generate_rook_moves(board, side_to_move, move_list, index);
    generate_bishop_moves(board, side_to_move, move_list, index);
    generate_queen_moves(board, side_to_move, move_list, index);
}

fn move_dest_empty_or_capture(board: &NormalBoard, side_to_move: u8, piece_dest: u16) -> bool {
    let piece_at_dest = board.at_sq(piece_dest as u8);
    (piece_at_dest == Piece::NoPiece as u8) || (Piece::color_of(piece_at_dest) != side_to_move)
}