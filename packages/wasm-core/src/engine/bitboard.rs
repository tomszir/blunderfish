use crate::engine::constants::{Color, Piece};

use super::bit::Bit;

#[derive(Copy, Clone)]
pub struct BitBoard {
    pub pieces: [[u64; 6]; 2],
}

pub struct MoveTable;

impl MoveTable {
    pub const KNIGHT_MOVES: [u64; 64] = Self::generate_knight_moves();

    // https://www.chessprogramming.org/Knight_Pattern
    pub const fn generate_knight_moves() -> [u64; 64] {
        let mut i = 0;
        let mut moves = [0; 64];

        while i < 64 {
            let b = 1_u64 << i;

            // Top-Left
            moves[i] |= (b & !(BitBoard::FILE_H)) << 17;
            moves[i] |= (b & !(BitBoard::FILE_G | BitBoard::FILE_H)) << 10;

            // Bottom-Left
            moves[i] |= (b & !(BitBoard::FILE_G | BitBoard::FILE_H)) >> 6;
            moves[i] |= (b & !(BitBoard::FILE_H)) >> 15;

            // Top-Right
            moves[i] |= (b & !(BitBoard::FILE_A)) << 15;
            moves[i] |= (b & !(BitBoard::FILE_A | BitBoard::FILE_B)) << 6;

            // Bottom-Right
            moves[i] |= (b & !(BitBoard::FILE_A | BitBoard::FILE_A)) >> 10;
            moves[i] |= (b & !(BitBoard::FILE_A)) >> 17;

            i += 1;
        }

        moves
    }
}

// Little-Endian Rank-File Mapping
impl BitBoard {
    pub const FILE_A: u64 = 0x0101010101010101;
    pub const FILE_B: u64 = Self::FILE_A << 1;
    pub const FILE_C: u64 = Self::FILE_A << 2;
    pub const FILE_D: u64 = Self::FILE_A << 3;
    pub const FILE_E: u64 = Self::FILE_A << 4;
    pub const FILE_F: u64 = Self::FILE_A << 5;
    pub const FILE_G: u64 = Self::FILE_A << 6;
    pub const FILE_H: u64 = Self::FILE_A << 7;

    pub const RANK_1: u64 = 0x00000000000000FF;
    pub const RANK_2: u64 = Self::RANK_1 << 8;
    pub const RANK_3: u64 = Self::RANK_1 << 16;
    pub const RANK_4: u64 = Self::RANK_1 << 24;
    pub const RANK_5: u64 = Self::RANK_1 << 32;
    pub const RANK_6: u64 = Self::RANK_1 << 40;
    pub const RANK_7: u64 = Self::RANK_1 << 48;
    pub const RANK_8: u64 = Self::RANK_1 << 56;

    pub const DIAGONAL: u64 = 0x8040201008040201; // A1-H8
    pub const ANTI_DIAGONAL: u64 = 0x0102040810204080; // H1-A8

    pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
    pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;

    pub fn empty() -> Self {
        Self {
            pieces: [[0; 6]; 2],
        }
    }

    pub fn set_piece(&mut self, color: usize, piece: usize, rank: usize, file: usize) {
        Bit::set(&mut self.pieces[color][piece], (8 * rank + file) as u64);
    }

    pub fn get_piece(&self, rank: usize, file: usize) -> Option<(usize, usize)> {
        let index = (8 * rank + file) as u64;

        for color in 0..=1 {
            for piece in 0..=5 {
                if Bit::get(self.pieces[color][piece], index) == 1 {
                    return Some((color, piece));
                }
            }
        }

        None
    }

    pub fn print(&self) {
        for rank in 0..=7 {
            for file in 0..=7 {
                let symbol = match self.get_piece(7 - rank, file) {
                    Some(color_piece) => match color_piece {
                        (Color::BLACK, Piece::PAWN) => "p",
                        (Color::BLACK, Piece::KNIGHT) => "n",
                        (Color::BLACK, Piece::BISHOP) => "b",
                        (Color::BLACK, Piece::ROOK) => "r",
                        (Color::BLACK, Piece::QUEEN) => "q",
                        (Color::BLACK, Piece::KING) => "k",
                        (Color::WHITE, Piece::PAWN) => "P",
                        (Color::WHITE, Piece::KNIGHT) => "N",
                        (Color::WHITE, Piece::BISHOP) => "B",
                        (Color::WHITE, Piece::ROOK) => "R",
                        (Color::WHITE, Piece::QUEEN) => "Q",
                        (Color::WHITE, Piece::KING) => "K",
                        _ => "?",
                    },
                    None => ".",
                };

                print!(" {} ", symbol);
            }
            println!();
        }
    }
}

pub fn print_bitboard(board: u64) {
    for rank in 0..=7 {
        for file in 0..=7 {
            let symbol = match Bit::get(board, (8 * (7 - rank) + file) as u64) {
                0 => ".",
                1 => "1",
                _ => "?",
            };

            print!(" {} ", symbol);
        }
        println!();
    }
}
