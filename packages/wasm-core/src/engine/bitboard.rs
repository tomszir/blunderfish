use regex::Regex;

use crate::engine::constants::{Color, Piece};

#[derive(Copy, Clone)]
pub struct BitBoard {
    pub sides: [u64; 2],
    pub pieces: [[u64; 6]; 2],
}

impl BitBoard {
    const FEN_RANK_REGEX: &'static str = r"([prnbqkbnrPRNBQKBNR1-8]{1,8})/?";

    pub fn empty() -> Self {
        Self {
            sides: [0; 2],
            pieces: [[0; 6]; 2],
        }
    }

    pub fn from_fen_ranks(fen_ranks: Option<&str>) -> Result<BitBoard, &'static str> {
        let mut bitboard = BitBoard::empty();
        let pattern = format!("^{}$", Self::FEN_RANK_REGEX.repeat(8));
        let ranks = fen_ranks.ok_or("FenParser: Incomplete FEN provided.")?;
        let regex = match Regex::new(pattern.as_str()) {
            Ok(regex) => regex,
            Err(_) => return Err("FenParser: An error occured while creating RankPattern Regex."),
        };
        let captures = regex
            .captures(ranks)
            .ok_or("FenParser: An errror occured while parsing ranks.")?;

        for rank in 0..=7 {
            let capture = captures.get(rank + 1);
            let fen_rank = capture.unwrap().as_str();
            bitboard.set_pieces_from_fen_rank(rank, fen_rank);
        }

        Ok(bitboard)
    }

    pub fn set_pieces_from_fen_rank(&mut self, rank: usize, fen_rank: &str) {
        let mut file = 0;

        for character in fen_rank.chars() {
            let color = if char::is_ascii_uppercase(&character) {
                Color::WHITE
            } else {
                Color::BLACK
            };

            let piece: Option<usize> = match character {
                'p' | 'P' => Some(Piece::PAWN),
                'n' | 'N' => Some(Piece::KNIGHT),
                'b' | 'B' => Some(Piece::BISHOP),
                'r' | 'R' => Some(Piece::ROOK),
                'q' | 'Q' => Some(Piece::QUEEN),
                'k' | 'K' => Some(Piece::KING),
                '1'..='8' | _ => None,
            };

            match piece {
                Some(piece) => self.set_piece(color, piece, rank, file),
                None => file += char::to_digit(character, 10).unwrap() as usize,
            }

            if char::is_alphabetic(character) {
                file += 1;
            }
        }
    }

    pub fn set_piece(&mut self, color: usize, piece: usize, rank: usize, file: usize) {
        let mask = (1 << ((7 - rank) * 8)) << (7 - file);
        self.sides[color] |= mask;
        self.pieces[color][piece] |= mask;
    }

    pub fn get_piece(&self, rank: usize, file: usize) -> Option<(usize, usize)> {
        let mask = 63 - rank * 8 - file;

        for color in 0..=1 {
            for piece in 0..=5 {
                if ((self.pieces[color][piece] >> mask) & 1) == 1 {
                    return Some((color, piece));
                }
            }
        }

        None
    }

    pub fn print(&self) {
        for rank in 0..=7 {
            for file in 0..=7 {
                let symbol = match self.get_piece(rank, file) {
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
