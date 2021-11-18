use chess::{Board, MoveGen};
use shakmaty::{fen::Fen, perft as sperft, Chess};
use std::str::FromStr;
use std::time::Instant;

pub fn perform_perft(fen: String, depth: u64, chess: bool, shakmaty: bool) {
    if chess {
        let board = Board::from_str(&fen).unwrap();
        let start = Instant::now();
        let result = MoveGen::movegen_perft_test(&board, depth as usize) as u64;
        let duration = start.elapsed();
        println!(
            "chess   : Perft {} of {}\tResult: {}\tTime: {}s {}ms",
            depth,
            fen,
            result,
            duration.as_secs(),
            duration.subsec_nanos() / 1000000
        );
    }
    if shakmaty {
        let setup: Fen = fen.parse().expect("invaild fen");
        let position: Chess = setup.position().expect("invalid setup");
        let start = Instant::now();
        let result = sperft(&position, depth as u32);
        let duration = start.elapsed();
        println!(
            "shakmaty: Perft {} of {}\tResult: {}\tTime: {}s {}ms",
            depth,
            fen,
            result,
            duration.as_secs(),
            duration.subsec_nanos() / 1000000
        );
    }
}
