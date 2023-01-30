use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Color;

impl Color {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

#[wasm_bindgen]
pub struct Piece;

impl Piece {
    pub const PAWN: usize = 0;
    pub const KNIGHT: usize = 1;
    pub const BISHOP: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

#[wasm_bindgen]
pub struct CastlingAvailability;

impl CastlingAvailability {
    const WHITE_QUEEN_SIDE: u8 = 0;
    const WHITE_KING_SIDE: u8 = 1;
    const BLACK_QUEEN_SIDE: u8 = 2;
    const BLACK_KING_SIDE: u8 = 3;
}
