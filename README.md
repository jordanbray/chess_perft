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

## Example

```bash
[jordan@localhost chess_perft]$ ./target/release/chess_perft -f "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1" -d 5 # Test the KiwiPete Position
Perft 5 of r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
Result: 193690690, Time: 1s 531ms
[jordan@localhost chess_perft]$ 
```

