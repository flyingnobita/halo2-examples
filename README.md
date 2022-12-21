# Halo2 Examples

This repo includes a few simple examples to illustrate how to write circuit in Halo2.

## Instruction

Compile the repo

```
cargo build
```

Run examples
```
cargo test -- --nocapture fibonacci_example1
cargo test -- --nocapture fibonacci_example2
cargo test -- --nocapture fibonacci_example3
cargo test -- --nocapture fibonacci_example4
cargo test -- --nocapture test_range_check_1
cargo test -- --nocapture test_range_check_2
cargo test -- --nocapture test_range_check_3
```

Plot the circuit layout
```
cargo test --all-features -- --nocapture plot_fibonacci1
```
