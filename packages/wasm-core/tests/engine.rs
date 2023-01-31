use wasm_core::engine::{
    bitboard::{print_bitboard, MoveTable},
    constants::Square,
    fen::FenParser,
};

#[test]
fn bitboard_from_fen() {
    let bitboard =
        FenParser::parse_ranks(Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")).unwrap();
    bitboard.print();
    print_bitboard(MoveTable::KNIGHT_MOVES[Square::B1 as usize]);
}
