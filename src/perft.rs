use shakmaty::{fen::Fen, perft as sperft, Chess};
use chessie::Game;
use cozy_chess::Board;
use std::str::FromStr;
use std::time::SystemTime;

/// See https://docs.rs/chessie/2.0.0/chessie/
fn cperft(game: &Game, depth: u64) -> u64 {
    // Recursion limit; return 1, since we're fathoming this node.
    if depth == 0 {
        return 1;
    }

    // Recursively accumulate the nodes from the remaining depths
    game.get_legal_moves().into_iter().fold(0, |nodes, mv| {
        nodes + cperft(&game.with_move_made(mv), depth - 1)
    })
}

/// See https://github.com/analog-hors/cozy-chess/blob/master/cozy-chess/examples/perft.rs
fn ccperft(board: &Board, depth: u64) -> u64 {
    if depth == 0 {
        1
    } else {
        let mut nodes = 0;
        board.generate_moves(|moves| {
            for mv in moves {
                let mut board = board.clone();
                board.play_unchecked(mv);
                nodes += ccperft(&board, depth - 1);
            }
            false
        });
        nodes
    }
}

pub fn perform_perft(fen: String, depth: u64, chess: bool, chess_nightly: bool, shakmaty: bool, chessie: bool, cozy_chess: bool) {
    if chess {
        let board = chess::Board::from_str(&fen).unwrap();
        let start = SystemTime::now();
        let result = chess::MoveGen::movegen_perft_test(&board, depth as usize) as u64;
        let duration = SystemTime::now().duration_since(start);
        match duration {
            Ok(clock) => {
                println!(
                    "chess         : Perft {} of {}\tResult: {}\tTime: {}s {}ms",
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
    if chess_nightly {
        let board = chess_nightly::Board::from_str(&fen).unwrap();
        let start = SystemTime::now();
        let result = chess_nightly::MoveGen::movegen_perft_test(&board, depth as usize) as u64;
        let duration = SystemTime::now().duration_since(start);
        match duration {
            Ok(clock) => {
                println!(
                    "chess_nightly : Perft {} of {}\tResult: {}\tTime: {}s {}ms",
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
                    "shakmaty      : Perft {} of {}\tResult: {}\tTime: {}s {}ms",
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
    if chessie {
        let setup = Game::from_fen(&fen).unwrap();
        let start = SystemTime::now();
        let result = cperft(&setup, depth);
        let duration = SystemTime::now().duration_since(start);
        match duration {
            Ok(clock) => {
                println!(
                    "chessie       : Perft {} of {}\tResult: {}\tTime: {}s {}ms",
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
    if cozy_chess {
        let setup = fen.parse::<Board>().unwrap();
        let start = SystemTime::now();
        let result = ccperft(&setup, depth);
        let duration = SystemTime::now().duration_since(start);
        match duration {
            Ok(clock) => {
                println!(
                    "cozy-chess    : Perft {} of {}\tResult: {}\tTime: {}s {}ms",
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
