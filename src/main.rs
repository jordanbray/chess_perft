extern crate chess;
extern crate getopts;

use getopts::Options;
use chess::{Board, construct};
use std::time::SystemTime;
use std::env;

fn perform_perft(fen: String, depth: u64, cache_size: usize) {
    let board = Board::from_fen(fen.to_owned()).unwrap();
    let start = SystemTime::now();
    let result = if cache_size == 0 { board.perft(depth) } else { board.perft_cache(depth, cache_size) };
    let duration = SystemTime::now().duration_since(start);
    match duration {
        Ok(clock) => {
            println!("Perft {} of {}\nResult: {}, Time: {}s {}ms", depth, fen, result, clock.as_secs(), clock.subsec_nanos() / 1000000);
        }, Err(_) => {
            panic!();
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned();
    let mut depth = 6;
    let mut cache = 0;

    let mut opts = Options::new();
    opts.optopt("f", "fen", "set the FEN to perfromt the perft on.", "FEN");
    opts.optopt("d", "depth", "set the depth to process the perft.", "DEPTH");
    opts.optopt("c", "cache", "set the cache size to a particular value.  Use 0 for no cache.", "CACHE_SIZE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    match matches.opt_str("f") {
        None => {},
        Some(x) => fen = x.to_owned(),
    }

    match matches.opt_str("d") {
        None => {},
        Some(x) => depth = x.parse::<u64>().unwrap()
    }

    match matches.opt_str("c") {
        None => {},
        Some(x) => cache = x.parse::<usize>().unwrap()
    }

    construct();

    perform_perft(fen, depth, cache);
}
