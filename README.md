# Perft Test Benchmarks for crates.io/chess/

This simple program tests the performance of both the 'chess' crate and the 'shakmaty' crate.

## Compiling

Compiling this application should be as easy as:

```bash
cargo build --release
```

## Running

To run the program, you must specify a valid FEN string (representing a chess position) and a depth to search.

The program will report the number of leaf nodes it found starting at that position, and the amount of time it took to search (ignoring setup time, which currently is a lot).

You can also use the -a [size] option to cache results at each depth, speeding up perft performance.

You can also use the -m option to use the MoveGen structure, which is an iterator to test its functionality.

You can also specify the -c option to use the 'chess' crate, or the -s option to use the 'shakmaty' crate.  Note: not all of the features supported by this application are in the 'shakmaty' crate, so I default to calling their 'perft' function for all command-line arguments.

# Performance

Below I compare the performance numbers (using the command 'RUSTFLAGS="-C target-cpu=native" cargo bench | python graph_benches.py') of shakmaty and chess (lower is better).

![Performance Numbers SVG](./performance.svg)

## Example

```bash
[jordan@blade chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 5 # Test the KiwiPete Position
Perft 5 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 193690690, Time: 0s 885ms
[jordan@blade chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 5 -c 65536 # Test the KiwiPete Position With Cache
Perft 5 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 193690690, Time: 0s 579ms
[jordan@blade chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 6 # Test the KiwiPete Position at Higher Depth
Perft 6 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 8031647685, Time: 39s 474ms
[jordan@blade chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 6 -c 33554432 # Test the KiwiPete Position at Higher Depth With Cache
Perft 6 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 8031647685, Time: 14s 186ms
[jordan@blade chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 5 -m # Test the KiwiPete Position with movegen
Perft 5 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 193690690, Time: 0s 867ms
[jordan@blade chess_perft]$

```

