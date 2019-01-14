#[macro_use]
extern crate bencher;
extern crate chess;
extern crate shakmaty;
extern crate chess_move_gen;

use bencher::Bencher;
use chess::{Board, MoveGen};

use shakmaty::Chess;
use shakmaty::fen::Fen;
use shakmaty::{perft as sperft};
use chess_move_gen::{Position, perft as cperft};

// This is a helper function to remove boilerplate code from all the perft_* _chess_incrementalbenchmarks
fn movegen_perft(bench: &mut Bencher, fen: String, depth: usize, count: usize) {
     let pos = Board::from_fen(fen).expect("Valid FEN");

     bench.iter(|| assert_eq!(MoveGen::movegen_perft_test(&pos, depth), count));
}

fn shakmaty_perft(bench: &mut Bencher, fen: String, depth: u64, count: u64) {
    let pos: Chess = fen.parse::<Fen>().expect("Invalid FEN").position().expect("Invalid Setup");

    bench.iter(|| assert_eq!(sperft(&pos, depth as u8), count));
}

fn chess_movegen_perft(bench: &mut Bencher, fen: String, depth: u64, count: usize) {
    let mut pos = Position::from_fen(&fen).expect("Valid FEN");

    bench.iter(|| assert_eq!(cperft(&mut pos, depth as usize, false, 0), count));
}

// These first two contain a technically invalid FEN position
// The position is completely valid, except it cannot be reached by any set of legal moves.
// Many chess move generators fail here due to a particular en-passant optimization.
// Should these two test ever fail, it should fail with an invaild FEN error, not an incorrect
// move count.

// Movegen Struct Tests.  Same as above

fn perft_01_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1".to_owned(), 6, 824064); // Invalid FEN
}

fn perft_02_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1".to_owned(), 6, 824064); // Invalid FEN
}

fn perft_03_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1".to_owned(), 6, 1440467);
}

fn perft_04_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1".to_owned(), 6, 1440467);
}

fn perft_05_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "5k2/8/8/8/8/8/8/4K2R w K - 0 1".to_owned(), 6, 661072);
}

fn perft_06_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "4k2r/8/8/8/8/8/8/5K2 b k - 0 1".to_owned(), 6, 661072);
}

fn perft_07_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1".to_owned(), 6, 803711);
}

fn perft_08_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "r3k3/8/8/8/8/8/8/3K4 b q - 0 1".to_owned(), 6, 803711);
}

fn perft_09_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1".to_owned(), 4, 1274206);
}

fn perft_10_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1".to_owned(), 4, 1274206);
}

fn perft_11_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1".to_owned(), 4, 1720476);
}

fn perft_12_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1".to_owned(), 4, 1720476);
}

fn perft_13_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1".to_owned(), 6, 3821001);
}

fn perft_14_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1".to_owned(), 6, 3821001);
}

fn perft_15_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1".to_owned(), 5, 1004658);
}

fn perft_16_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1".to_owned(), 5, 1004658);
}

fn perft_17_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "4k3/1P6/8/8/8/8/K7/8 w - - 0 1".to_owned(), 6, 217342);
}

fn perft_18_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/k7/8/8/8/8/1p6/4K3 b - - 0 1".to_owned(), 6, 217342);
}

fn perft_19_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/P1k5/K7/8/8/8/8/8 w - - 0 1".to_owned(), 6, 92683);
}

fn perft_20_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/8/8/8/8/k7/p1K5/8 b - - 0 1".to_owned(), 6, 92683);
}

fn perft_21_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "K1k5/8/P7/8/8/8/8/8 w - - 0 1".to_owned(), 6, 2217);
}

fn perft_22_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/8/8/8/8/p7/8/k1K5 b - - 0 1".to_owned(), 6, 2217);
}

fn perft_23_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/k1P5/8/1K6/8/8/8/8 w - - 0 1".to_owned(), 7, 567584);
}

fn perft_24_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/8/8/8/1k6/8/K1p5/8 b - - 0 1".to_owned(), 7, 567584);
}

fn perft_25_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1".to_owned(), 4, 23527);
}

fn perft_26_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1".to_owned(), 4, 23527);
}

fn perft_kiwipete_chess_incremental(bench: &mut Bencher) {
    movegen_perft(bench, "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_owned(), 4, 4085603);
}



// shakmaty Struct Tests.  Same as above

fn perft_01_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1".to_owned(), 6, 824064); // Invalid FEN
}

fn perft_02_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1".to_owned(), 6, 824064); // Invalid FEN
}

fn perft_03_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1".to_owned(), 6, 1440467);
}

fn perft_04_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1".to_owned(), 6, 1440467);
}

fn perft_05_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "5k2/8/8/8/8/8/8/4K2R w K - 0 1".to_owned(), 6, 661072);
}

fn perft_06_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "4k2r/8/8/8/8/8/8/5K2 b k - 0 1".to_owned(), 6, 661072);
}

fn perft_07_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1".to_owned(), 6, 803711);
}

fn perft_08_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "r3k3/8/8/8/8/8/8/3K4 b q - 0 1".to_owned(), 6, 803711);
}

fn perft_09_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1".to_owned(), 4, 1274206);
}

fn perft_10_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1".to_owned(), 4, 1274206);
}

fn perft_11_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1".to_owned(), 4, 1720476);
}

fn perft_12_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1".to_owned(), 4, 1720476);
}

fn perft_13_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1".to_owned(), 6, 3821001);
}

fn perft_14_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1".to_owned(), 6, 3821001);
}

fn perft_15_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1".to_owned(), 5, 1004658);
}

fn perft_16_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1".to_owned(), 5, 1004658);
}

fn perft_17_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "4k3/1P6/8/8/8/8/K7/8 w - - 0 1".to_owned(), 6, 217342);
}

fn perft_18_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/k7/8/8/8/8/1p6/4K3 b - - 0 1".to_owned(), 6, 217342);
}

fn perft_19_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/P1k5/K7/8/8/8/8/8 w - - 0 1".to_owned(), 6, 92683);
}

fn perft_20_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/8/8/8/8/k7/p1K5/8 b - - 0 1".to_owned(), 6, 92683);
}

fn perft_21_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "K1k5/8/P7/8/8/8/8/8 w - - 0 1".to_owned(), 6, 2217);
}

fn perft_22_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/8/8/8/8/p7/8/k1K5 b - - 0 1".to_owned(), 6, 2217);
}

fn perft_23_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/k1P5/8/1K6/8/8/8/8 w - - 0 1".to_owned(), 7, 567584);
}

fn perft_24_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/8/8/8/1k6/8/K1p5/8 b - - 0 1".to_owned(), 7, 567584);
}

fn perft_25_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1".to_owned(), 4, 23527);
}

fn perft_26_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1".to_owned(), 4, 23527);
}

fn perft_kiwipete_shakmaty(bench: &mut Bencher) {
    shakmaty_perft(bench, "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_owned(), 4, 4085603);
}

// Perft tests for the chess_move_gen crate

fn perft_01_chess_move_gen(bench: &mut Bencher) {
    bench.iter(|| assert_eq!(0, 0));
}


fn perft_02_chess_move_gen(bench: &mut Bencher) {
    bench.iter(|| assert_eq!(0, 0));
}

fn perft_03_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1".to_owned(), 6, 1440467);
}

fn perft_04_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1".to_owned(), 6, 1440467);
}

fn perft_05_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "5k2/8/8/8/8/8/8/4K2R w K - 0 1".to_owned(), 6, 661072);
}

fn perft_06_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "4k2r/8/8/8/8/8/8/5K2 b k - 0 1".to_owned(), 6, 661072);
}

fn perft_07_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1".to_owned(), 6, 803711);
}

fn perft_08_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "r3k3/8/8/8/8/8/8/3K4 b q - 0 1".to_owned(), 6, 803711);
}

fn perft_09_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1".to_owned(), 4, 1274206);
}

fn perft_10_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1".to_owned(), 4, 1274206);
}

fn perft_11_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1".to_owned(), 4, 1720476);
}

fn perft_12_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1".to_owned(), 4, 1720476);
}

fn perft_13_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1".to_owned(), 6, 3821001);
}

fn perft_14_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1".to_owned(), 6, 3821001);
}

fn perft_15_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1".to_owned(), 5, 1004658);
}

fn perft_16_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1".to_owned(), 5, 1004658);
}

fn perft_17_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "4k3/1P6/8/8/8/8/K7/8 w - - 0 1".to_owned(), 6, 217342);
}

fn perft_18_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/k7/8/8/8/8/1p6/4K3 b - - 0 1".to_owned(), 6, 217342);
}

fn perft_19_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/P1k5/K7/8/8/8/8/8 w - - 0 1".to_owned(), 6, 92683);
}

fn perft_20_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/8/8/8/8/k7/p1K5/8 b - - 0 1".to_owned(), 6, 92683);
}

fn perft_21_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "K1k5/8/P7/8/8/8/8/8 w - - 0 1".to_owned(), 6, 2217);
}

fn perft_22_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/8/8/8/8/p7/8/k1K5 b - - 0 1".to_owned(), 6, 2217);
}

fn perft_23_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/k1P5/8/1K6/8/8/8/8 w - - 0 1".to_owned(), 7, 567584);
}

fn perft_24_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/8/8/8/1k6/8/K1p5/8 b - - 0 1".to_owned(), 7, 567584);
}

fn perft_25_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1".to_owned(), 4, 23527);
}

fn perft_26_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1".to_owned(), 4, 23527);
}

fn perft_kiwipete_chess_move_gen(bench: &mut Bencher) {
    chess_movegen_perft(bench, "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_owned(), 4, 4085603);
}

benchmark_group!(
    benches,
    perft_01_shakmaty,
    perft_02_shakmaty,
    perft_03_shakmaty,
    perft_04_shakmaty,
    perft_05_shakmaty,
    perft_06_shakmaty,
    perft_07_shakmaty,
    perft_08_shakmaty,
    perft_09_shakmaty,
    perft_10_shakmaty,
    perft_11_shakmaty,
    perft_12_shakmaty,
    perft_13_shakmaty,
    perft_14_shakmaty,
    perft_15_shakmaty,
    perft_16_shakmaty,
    perft_17_shakmaty,
    perft_18_shakmaty,
    perft_19_shakmaty,
    perft_20_shakmaty,
    perft_21_shakmaty,
    perft_22_shakmaty,
    perft_23_shakmaty,
    perft_24_shakmaty,
    perft_25_shakmaty,
    perft_26_shakmaty,
    perft_kiwipete_shakmaty,
    perft_01_chess_incremental,
    perft_02_chess_incremental,
    perft_03_chess_incremental,
    perft_04_chess_incremental,
    perft_05_chess_incremental,
    perft_06_chess_incremental,
    perft_07_chess_incremental,
    perft_08_chess_incremental,
    perft_09_chess_incremental,
    perft_10_chess_incremental,
    perft_11_chess_incremental,
    perft_12_chess_incremental,
    perft_13_chess_incremental,
    perft_14_chess_incremental,
    perft_15_chess_incremental,
    perft_16_chess_incremental,
    perft_17_chess_incremental,
    perft_18_chess_incremental,
    perft_19_chess_incremental,
    perft_20_chess_incremental,
    perft_21_chess_incremental,
    perft_22_chess_incremental,
    perft_23_chess_incremental,
    perft_24_chess_incremental,
    perft_25_chess_incremental,
    perft_26_chess_incremental,
    perft_kiwipete_chess_incremental,
    perft_01_chess_move_gen,
    perft_02_chess_move_gen,
    perft_03_chess_move_gen,
    perft_04_chess_move_gen,
    perft_05_chess_move_gen,
    perft_06_chess_move_gen,
    perft_07_chess_move_gen,
    perft_08_chess_move_gen,
    perft_09_chess_move_gen,
    perft_10_chess_move_gen,
    perft_11_chess_move_gen,
    perft_12_chess_move_gen,
    perft_13_chess_move_gen,
    perft_14_chess_move_gen,
    perft_15_chess_move_gen,
    perft_16_chess_move_gen,
    perft_17_chess_move_gen,
    perft_18_chess_move_gen,
    perft_19_chess_move_gen,
    perft_20_chess_move_gen,
    perft_21_chess_move_gen,
    perft_22_chess_move_gen,
    perft_23_chess_move_gen,
    perft_24_chess_move_gen,
    perft_25_chess_move_gen,
    perft_26_chess_move_gen,
    perft_kiwipete_chess_move_gen);

benchmark_main!(benches);
