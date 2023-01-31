use wasm_bindgen::prelude::wasm_bindgen;

use super::{bitboard::BitBoard, fen::FenParser};

pub type Position = (u8, u8);

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct BoardState {
    pieces: BitBoard,
    move_history: u8,
    active_color: usize,
    castling_availability: u8,
    en_passant: Position,
    half_move_clock: u8,
    full_move_clock: u8,
}

pub type BoardStateTuple = (BitBoard, usize, u8, Position, u8, u8);

#[wasm_bindgen]
impl BoardState {
    pub fn default() -> Self {
        Self {
            pieces: BitBoard::empty(),
            active_color: 0,
            castling_availability: 0,
            en_passant: (0, 0),
            half_move_clock: 0,
            full_move_clock: 0,
            move_history: 0,
        }
    }

    pub fn from_fen(fen: &str) -> Option<BoardState> {
        match FenParser::parse(fen) {
            Ok((
                pieces,
                active_color,
                castling_availability,
                en_passant,
                half_move_clock,
                full_move_clock,
            )) => Some(BoardState {
                pieces,
                active_color,
                castling_availability,
                en_passant,
                half_move_clock,
                full_move_clock,
                move_history: 0,
            }),
            Err(_) => None,
        }
    }
}
