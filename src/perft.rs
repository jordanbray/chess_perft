use chess::{Board, MoveGen};
use shakmaty::{fen::Fen, perft as sperft, Chess};
use std::str::FromStr;
use std::time::SystemTime;

pub fn perform_perft(fen: String, depth: u64, chess: bool, shakmaty: bool) {
    if chess {
        let board = Board::from_str(&fen).unwrap();
        let start = SystemTime::now();
        let result = MoveGen::movegen_perft_test(&board, depth as usize) as u64;
        let duration = SystemTime::now().duration_since(start);
        match duration {
            Ok(clock) => {
                println!(
                    "chess   : Perft {} of {}\tResult: {}\tTime: {}s {}ms",
                    depth,
                    fen,
                    result,
                    clock.as_secs(),
                    clock.subsec_nanos() / 1000000
                );
            }
            Err(_) => {
                panic!();
            }
        }
    }
    if shakmaty {
        let setup: Fen = fen.parse().expect("invaild fen");
        let position: Chess = setup
            .0
            .position(shakmaty::CastlingMode::Standard)
            .expect("invalid setup");
        let start = SystemTime::now();
        let result = sperft(&position, depth as u32);
        let duration = SystemTime::now().duration_since(start);
        match duration {
            Ok(clock) => {
                println!(
                    "shakmaty: Perft {} of {}\tResult: {}\tTime: {}s {}ms",
                    depth,
                    fen,
                    result,
                    clock.as_secs(),
                    clock.subsec_nanos() / 1000000
                );
            }
            Err(_) => {
                panic!();
            }
        }
    }
}
