extern crate chess;
extern crate getopts;
extern crate shakmaty;

use getopts::Options;
use std::env;

mod perft;
use crate::perft::perform_perft;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned();
    let mut depth = 6;
    let mut chess = true;
    let mut shakmaty = false;

    let mut opts = Options::new();
    opts.optopt("f", "fen", "set the FEN to perfromt the perft on.", "FEN");
    opts.optopt("d", "depth", "set the depth to process the perft.", "DEPTH");
    opts.optflag("c", "chess", "use the 'chess' library only (default).");
    opts.optflag("s", "shakmaty", "use the 'shakmaty' library only.");
    opts.optflag("a", "all", "use all supported libraries library.");
    opts.optflag("h", "help", "print this help menu.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    match matches.opt_str("f") {
        None => {}
        Some(x) => fen = x.to_owned(),
    }

    match matches.opt_str("d") {
        None => {}
        Some(x) => depth = x.parse::<u64>().unwrap(),
    }

    if matches.opt_present("c") {
        chess = true;
        shakmaty = false;
    }
    if matches.opt_present("s") {
        chess = false;
        shakmaty = true;
    }
    if matches.opt_present("a") {
        chess = true;
        shakmaty = true;
    }

    perform_perft(fen, depth, chess, shakmaty);
}
