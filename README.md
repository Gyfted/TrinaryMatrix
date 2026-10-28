# TrinaryMatrix

A minimal Rust **host simulator** for matrix-vector inference with mathematical
weights in `{-1, 0, +1}`. Negative weights subtract, zero weights skip, and positive
weights add an integer activation. An independent multiplication implementation
checks the result on the same binary host.

This is a public, generic mathematical scaffold. It contains synthetic inputs
and no device design, process parameters, trained model, unpublished architecture,
or confidential research. It does not implement a native ternary processor or
establish a complete guest software stack.

## Run

With rustup installed, `rust-toolchain.toml` selects Rust 1.99.1 and the rustfmt
and Clippy components. The compiler toolchain includes Cargo and rustdoc. Run:

```sh
cargo run --locked --offline -- --check
cargo test --locked --offline
cargo fmt --all -- --check
cargo clippy --locked --offline --all-targets -- -D warnings
```

GitHub Actions runs these same checks on pull requests to `main` and pushes to
`main` or the scaffold development branch. The workflow uses a read-only
repository token and does not persist checkout credentials. Toolchain setup
requires a network connection; the dependency-free crate checks run offline.

The fixed demo multiplies a two-row matrix by `[4, -2, 3]` and checks `[1, -3]`:

```text
input: [4, -2, 3]
add/sub/skip output: [1, -3]
multiply reference: [1, -3]
check: passed (host simulation; no physical performance claim)
```

## Contract

- `Weight` admits only the three mathematical values; it defines no native
  packing, ABI, or target trit encoding.
- `Matrix::new` validates positive dimensions, checked shape multiplication,
  exact row-major weight count, and weight values, in that order.
- `Matrix::infer` accepts one host `i64` activation per column and returns one
  host `i64` result per row. It checks every partial sum in column order and
  returns an indexed error on overflow. A later cancellation does not repair
  an earlier overflow. Inputs and model remain unchanged on error.
- `Matrix::reference_infer` uses host `i128` multiplication and checks each
  prefix against the same `i64` limits. It is a correctness reference, not an
  optimized binary baseline.
- Rust allocation and process failures remain host concerns. Allocation failure
  is not a recoverable modeled error. There are no biases, quantization,
  training, nonlinear layers, model import, or persistent storage APIs yet.

`cargo test` includes the executable API documentation example as well as the
integration tests. Tests compare all 729 two-by-three ternary matrices against
all 125 vectors over `[-2, 2]`, exercise integer boundaries and invalid inputs,
and check the CLI's success and failure paths. No timing, energy, silicon
efficiency, or speedup is claimed. Both implementations execute on the
development host.

See [the coverage map](docs/TRINARYASM_COVERAGE.md) for explicit host/guest
boundaries and [the native-stack sequence](docs/NATIVE_STACK.md) for future gates.
The crate has no external dependencies and is marked `publish = false` to avoid
accidental registry publication.
