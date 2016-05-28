# Perft Test Benchmarks for crates.io/chess/

This simple program tests the performance of the rust chess move generator.

## Compiling

Compiling this application should be as easy as:

```bash
cargo build --release
```

## Running

To run the program, you must specify a valid FEN string (representing a chess position) and a depth to search.

The program will report the number of leaf nodes it found starting at that position, and the amount of time it took to search (ignoring setup time, which currently is a lot).

You can also use the -c [size] option to cache results at each depth, speeding up perft performance.

## Example

```bash
[jordan@localhost chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 5 # Test the KiwiPete Position
Perft 5 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 193690690, Time: 1s 528ms
[jordan@localhost chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 5 -c 65536 # Test the KiwiPete Position With Cache
Perft 5 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 193690690, Time: 0s 921ms
[jordan@localhost chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 6 # Test the KiwiPete Position at Higher Depth
Perft 6 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 8031647685, Time: 69s 217ms
[jordan@localhost chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 6 -c 33554432 # Test the KiwiPete Position at Higher Depth With Cache
Perft 6 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 8031647685, Time: 18s 667ms
[jordan@localhost chess_perft]$ 

```

