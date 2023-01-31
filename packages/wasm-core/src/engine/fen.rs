use regex::Regex;

use super::{
    bitboard::BitBoard,
    board_state::{BoardStateTuple, Position},
    constants::{CastlingAvailability, Color, Piece},
};

#[derive(Debug)]
pub enum FenParserError {
    Incomplete,
    Regex,
    Unknown,
}

pub struct FenParser;

// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
impl FenParser {
    const RANK_REGEX: &'static str = r"([prnbqkbnrPRNBQKBNR1-8]{1,8})/?";

    pub fn parse(fen: &str) -> Result<BoardStateTuple, FenParserError> {
        let mut split = fen.split_whitespace();

        let pieces = Self::parse_ranks(split.next())?;
        let active_color = Self::parse_active_color(split.next())?;
        let castling_availability = Self::parse_castling_availability(split.next())?;
        let en_passant = Self::parse_en_passant(split.next())?;
        let half_move_clock = Self::parse_move_clock(split.next())?;
        let full_move_clock = Self::parse_move_clock(split.next())?;

        Ok((
            pieces,
            active_color,
            castling_availability,
            en_passant,
            half_move_clock,
            full_move_clock,
        ))
    }

    pub fn parse_ranks(input: Option<&str>) -> Result<BitBoard, FenParserError> {
        let mut bitboard = BitBoard::empty();
        let pattern = format!("^{}$", Self::RANK_REGEX.repeat(8));
        let ranks = input.ok_or(FenParserError::Incomplete)?;
        let regex = match Regex::new(pattern.as_str()) {
            Ok(regex) => regex,
            Err(_) => return Err(FenParserError::Regex),
        };
        let captures = regex.captures(ranks).ok_or(FenParserError::Unknown)?;

        for rank in 0..=7 {
            let capture = captures.get(rank + 1);
            let fen_rank = capture.unwrap().as_str();
            Self::parse_rank(&mut bitboard, rank, fen_rank);
        }

        Ok(bitboard)
    }

    pub fn parse_rank(bitboard: &mut BitBoard, rank: usize, fen_rank: &str) {
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
                Some(piece) => bitboard.set_piece(color, piece, 7 - rank, file),
                None => file += char::to_digit(character, 10).unwrap() as usize,
            }

            if char::is_alphabetic(character) {
                file += 1;
            }
        }
    }

    pub fn parse_active_color(input: Option<&str>) -> Result<usize, FenParserError> {
        let color_string = input.ok_or(FenParserError::Incomplete)?;
        let color = match color_string {
            "w" => Color::WHITE,
            "b" => Color::BLACK,
            _ => return Err(FenParserError::Incomplete),
        };

        Ok(color)
    }

    pub fn parse_castling_availability(input: Option<&str>) -> Result<u8, FenParserError> {
        let castling_string = input.ok_or(FenParserError::Incomplete)?;
        let mut castling_availability: u8 = 0;

        for character in castling_string.chars() {
            castling_availability |= match character {
                '-' => return Ok(0),
                'Q' => CastlingAvailability::WHITE_QUEEN_SIDE,
                'q' => CastlingAvailability::BLACK_QUEEN_SIDE,
                'K' => CastlingAvailability::WHITE_KING_SIDE,
                'k' => CastlingAvailability::BLACK_KING_SIDE,
                _ => 0,
            }
        }

        Ok(castling_availability)
    }

    pub fn parse_en_passant(input: Option<&str>) -> Result<Position, FenParserError> {
        Ok((0, 0))
    }

    pub fn parse_move_clock(input: Option<&str>) -> Result<u8, FenParserError> {
        Ok(0)
    }
}
