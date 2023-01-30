use super::{
    bitboard::BitBoard,
    board::{BoardStateTuple, Position},
};

pub struct FenParser;

// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
impl FenParser {
    pub fn parse(fen: &str) -> Result<BoardStateTuple, &'static str> {
        let mut split = fen.split_whitespace();

        let pieces = BitBoard::from_fen_ranks(split.next())?;
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

    pub fn parse_active_color(input: Option<&str>) -> Result<usize, &'static str> {
        Ok(0) // TODO: Finish these
    }

    pub fn parse_castling_availability(input: Option<&str>) -> Result<u8, &'static str> {
        Ok(0)
    }

    pub fn parse_en_passant(input: Option<&str>) -> Result<Position, &'static str> {
        Ok((0, 0))
    }

    pub fn parse_move_clock(input: Option<&str>) -> Result<u8, &'static str> {
        Ok(0)
    }
}
