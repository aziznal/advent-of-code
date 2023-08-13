# Advent of Code 2022 / Rust

## Writing Solutions

I've organized things to where every solution is in its own file. `day1.rs`
corresponds to the first day's solution and so on.

Any input required for files is simply stored in the same directory, e.g. `day1-input.txt`

Every solution file has a `run` function. To run a solution, import it in `main.rs` and call its run function.

## Running

Use the following command to run in watch mode:

```bash
# run this once when you clone this repo
cargo install cargo-watch

# run this whenever you need
cargo watch -c -w src -x run
```

the `cargo watch` command clears the console and runs your main file whenever
you make a change and save it
