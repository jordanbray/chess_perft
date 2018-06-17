use std::time::SystemTime;

use chess::{Board,  MoveGen};
use shakmaty::{Chess, fen::Fen, perft};

pub fn perform_perft(fen: String, depth: u64, cache_size: usize, movegen: bool, chess: bool, shakmaty: bool) {
    if chess {
        let board = Board::from_fen(fen.to_owned()).unwrap();
        let start = SystemTime::now();
        let result = if movegen {
                MoveGen::movegen_perft_test(board, depth as usize) as u64
            } else if cache_size == 0 {
                board.perft(depth)
            } else {
                board.perft_cache(depth, cache_size)
            };
        let duration = SystemTime::now().duration_since(start);
        match duration {
            Ok(clock) => {
                println!("chess   : Perft {} of {}\tResult: {}\tTime: {}s {}ms", depth, fen, result, clock.as_secs(), clock.subsec_nanos() / 1000000);
            }, Err(_) => {
                panic!();
            }
        }
    }
    if shakmaty {
        let setup: Fen = fen.parse().expect("invaild fen");
        let position: Chess = setup.position().expect("invalid setup");
        let start = SystemTime::now();
        let result = perft(&position, depth as u8);
        let duration = SystemTime::now().duration_since(start);
        match duration {
            Ok(clock) => {
                println!("shakmaty: Perft {} of {}\tResult: {}\tTime: {}s {}ms", depth, fen, result, clock.as_secs(), clock.subsec_nanos() / 1000000);
            }, Err(_) => {
                panic!();
            }
        }
    }
}

