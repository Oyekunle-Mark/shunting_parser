# shunting_parser

An expression parser which uses the [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) for parsing arithmetic expressions.

## Usage

You can run the evaluator using the `cargo run` command like this:

```bash
$ cargo run eval "2+2"
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/shunting_parser eval 2+2`
> 4
$ cargo run eval "3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3"
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/shunting_parser eval '3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3'`
> 3.0001220703125

$ cargo run eval "((2*3) + pi) / 10"
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/shunting_parser eval '((2*3) + pi) / 10'`
> 0.9141592653589999

$ cargo run eval "3 + 4 * 2 / ( 1 - max(5, 2) ) ^ min(2,11) ^ 3"
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/shunting_parser eval '3 + 4 * 2 / ( 1 - max(5, 2) ) ^ min(2,11) ^ 3'`
> 3.0001220703125
```

## Changes

You can create a PR with your changes if you wish to make a change.
Existing functionalities are covered by test cases and you can use `cargo test` to assert that they work after changes.
