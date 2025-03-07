# Fuzzing Solana crates

## Prerequisites

To get started, you will need `cargo-afl`, which can be installed like so:

```sh
% cargo install cargo-afl
```

AFL++ will have some pretty beefy minimum system requirements — for this crate,
a Linux system with plenty of RAM, plenty of CPU cores, and plenty of
single-threaded CPU performance is recommended.

## Fuzzing

There is a shell script inside `sandbox/` that will handle basic environment
setup for `cargo-afl` as well as invoking the fuzzer on the crate.  To get a
more in-depth explanation of the steps involved, see [this tutorial][afl], but
to get started, simply run:

```sh
% ./fuzz.sh
```

This should be enough to configure AFL and its toolchain and begin fuzzing, but
if not `cargo-afl` and AFL++ may give additional instructions in order to
properly configure your system for fuzzing.

## Reproducing an example

`cargo-afl` provides a `run` subcommand for reproducing a specific test run,
for which there is a second shell script provided.  For instance, to test the
binary on a specific seed test case, run:

```sh
% ./run.sh in/my-case
```

To rerun a specific test case that crashed, simply run:

```sh
% ./run.sh out/default/crashes/id:...
```

## Fuzzing code from solana-core

The entry point for the `sandbox` crate uses the [`afl`] crate to construct
data for an input test case using the [`arbitrary`] crate.  To construct a
test, simply edit the definitions for `type Data` and `fn run` like so:

```rust
type Data = (MyInput1, MyInput2, ..);
fn run((input1, input2, ..): Data) {
    assert!(input1.do_something(input2));
}
```

As long as all input types in `Data` implement [`Arbitrary`][trait.Arbitrary],
rerunning AFL with `fuzz.sh` will begin fuzzing test cases with the new test.
Most importantly, AFL looks for crashes, so it is crucial to panic *if and only
if* a regression has occurred.

Furthermore, since AFL requires using a special instrumented toolchain to track
code coverage and explore different program branches, certain C/C++
dependencies may not build correctly and will need to be stubbed out (e.g.
`solana-ledger` depends on `rocksdb`, which has been stubbed out with a trivial
wrapper which immediately panics).

[afl]: https://rust-fuzz.github.io/book/afl.html "AFL tutorial from the Rust Fuzzing Book"
[`afl`]: https://docs.rs/afl
[`arbitrary`]: https://docs.rs/arbitrary
[trait.Arbitrary]: https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html
