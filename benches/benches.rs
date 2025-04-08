use bencher::benchmark_group;
use bencher::benchmark_main;
use bencher::Bencher;
use chess::{Board, MoveGen};
use chessie::Game as ChessieGame;
use cozy_chess::Board as CozyBoard;
use shakmaty::fen::Fen;
use shakmaty::perft as sperft;
use shakmaty::Chess;

use std::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/bench_macros.rs"));

#[macro_export]
macro_rules! gen_perft_inputs {
    () => {
        gen_perft_inputs_internal!();
    }
}

#[macro_export]
macro_rules! get_names {
    ($group_name:ident) => {
        get_names_internal!($group_name);
    };
}

// This is a helper function to remove boilerplate code from all the perft_* _chess_incrementalbenchmarks
fn movegen_perft(bench: &mut Bencher, fen: String, depth: usize, count: usize) {
    let pos = Board::from_str(&fen).expect("Valid FEN");

    bench.iter(|| assert_eq!(MoveGen::movegen_perft_test(&pos, depth), count));
}

fn nightly_movegen_perft(bench: &mut Bencher, fen: String, depth: usize, count: usize) {
    let pos = chess_nightly::Board::from_str(&fen).expect("Valid FEN");

    bench.iter(|| {
        assert_eq!(
            chess_nightly::MoveGen::movegen_perft_test(&pos, depth),
            count
        )
    });
}

fn shakmaty_perft(bench: &mut Bencher, fen: String, depth: u64, count: u64) {
    let pos: Chess = fen
        .parse::<Fen>()
        .expect("Invalid FEN")
        .0
        .position(shakmaty::CastlingMode::Standard)
        .expect("Invalid Setup");

    bench.iter(|| assert_eq!(sperft(&pos, depth as u32), count));
}

fn internal_cozy_perft(board: &CozyBoard, depth: u64) -> u64 {
    let mut nodes = 0;
    match depth {
        0 => nodes += 1,
        1 => {
            board.generate_moves(|moves| {
                nodes += moves.len() as u64;
                false
            });
        }
        _ => {
            board.generate_moves(|moves| {
                for mv in moves {
                    let mut board = board.clone();
                    board.play_unchecked(mv);
                    let child_nodes = internal_cozy_perft(&board, depth - 1);
                    nodes += child_nodes;
                }
                false
            });
        }
    }
    nodes
}

fn cozy_perft(bench: &mut Bencher, fen: String, depth: u64, count: u64) {
    let pos = CozyBoard::from_fen(&fen, false).unwrap();
    bench.iter(|| assert_eq!(internal_cozy_perft(&pos, depth), count));
}

fn internal_chessie_perft(game: &ChessieGame, depth: u64) -> u64 {
    match depth {
        0 => 1,
        1 => game.get_legal_moves().len() as u64,
        _ => game.get_legal_moves().into_iter().fold(0, |nodes, mv| {
            nodes + internal_chessie_perft(&game.with_move_made(mv), depth - 1)
        }),
    }
}

fn chessie_perft(bench: &mut Bencher, fen: String, depth: u64, count: u64) {
    let game = ChessieGame::from_fen(&fen).unwrap();
    bench.iter(|| assert_eq!(internal_chessie_perft(&game, depth), count));
}

// See perft_funcs.json
gen_perft_inputs!();

get_names!(benches);

benchmark_main!(benches);
