# Perft Test Benchmarks for crates.io/chess/, crates.io/shakmaty/

This simple program tests the performance of the 'chess' crate and the 'shakmaty' crate.  The 'chess-move-gen' crate was removed because it seems to be no-longer maintained (and I couldn't get it to compile recently).

## Running

To run the program, you must specify a valid FEN string (representing a chess position) and a depth to search.

The program will report the number of leaf nodes it found starting at that position, and the amount of time it took to search (ignoring setup time, which currently is a lot).

You can also use the -a [size] option to cache results at each depth, speeding up perft performance.

You can also use the -m option to use the MoveGen structure, which is an iterator to test its functionality.  (This is now the fastest way to generate moves, in most cases, for the 'chess' crate.)

You can also specify the -c option to use the 'chess' crate, the -s option to use the 'shakmaty' crate, or the -g for the 'chesss-move-gen' crate.  Note: not all of the features supported by this application are in the 'shakmaty' crate, so I default to calling their 'perft' function for all command-line arguments.

# Performance

Below I compare the performance numbers (using the command 'RUSTFLAGS="-C target-cpu=native" cargo bench | python graph_benches.py') of chess and shakmaty (lower is better).

![Performance Numbers SVG](./performance.svg)

If viewing on crates.io, you can view the performance numbers on https://github.com/jordanbray/chess_perft.

## Example

```bash
[jordan@razer chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 6 -m # Test the KiwiPete Position
chess   : Perft 6 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1       Result: 8031647685      Time: 22s 492ms
```

