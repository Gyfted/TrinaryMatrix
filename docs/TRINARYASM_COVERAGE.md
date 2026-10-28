# TrinaryASM coverage: public scaffold scope 0.1

Scope 0.1 consists only of the host mathematical simulator below. No guest
executable, assembler, native device, or TrinaryASM instruction sequence is
included. **H** means host-only evidence outside the guest boundary; **G** means
a native path remains a gap. Host success is not native execution evidence.

| API / feature | Current semantics and implementation | Success / failure evidence | Native path, limits, remaining gap |
|---|---|---|---|
| H01 `Weight::try_from` | Host `i8` values -1/0/+1 become typed mathematical weights; other values return `InvalidWeight`. | `all_weight_values_are_checked` covers all 256 host input values. | H; G: no target trit encoding, load, validation routine, or fault protocol. |
| H02 `Weight::value` | Host enum converts to mathematical host `i8`. No stable representation or storage packing is promised. | H01 round-trips all valid values. | H; G: target representation and external interchange boundary are undefined. |
| H03 `Matrix::new` | Positive host `usize` dimensions; checked product; exact row-major element count; H01 validates elements. Returns `ZeroDimension`, `ShapeOverflow`, `WeightCount`, or indexed `InvalidWeight`; host allocation failure is outside the model. | `constructor_rejects_invalid_shapes_and_values`; valid matrices in exhaustive and boundary tests; the `Matrix::infer` doctest constructs a model through the public API. | H; G: native word/index widths, memory layout/allocation, guest constructor and errors. |
| H04 `Matrix::infer` | Immutable host `i64` input; exact column count; per-row add/subtract/skip; every prefix must fit `i64`; indexed overflow returns no partial output. | `synthetic_fixture_and_zero_weight`, `exhaustive_small_matrices_match_independent_reference` (91,125 fixtures), `boundary_results_and_prefix_overflow_agree` (441 boundary fixtures plus explicit negation/prefix faults), `shape_mismatch_is_reported_before_execution`, `failure_does_not_mutate_model_or_input`; the public API doctest checks `[1]` and a short-input error through `cargo test`. | H; G: native activation/accumulator widths, checked arithmetic/overflow protocol, indexing/control, guest routine, encoded execution and fault evidence. Add/subtract/skip is a mathematical candidate only; no new opcode is proposed. |
| H05 `Matrix::reference_infer` | Separate host `i128` multiply-and-sum implementation checks each prefix fits host `i64`. Uses the same validated model but does not call H04. | H04 differential fixtures; explicit known output `[1, -3]`. | H verification helper; intentionally outside guest scope. Its existence supplies no native multiplication or inference capability. |
| H06 input-length validation | Private host helper shared by H04/H05 rejects any vector whose length differs from columns before arithmetic. | `shape_mismatch_is_reported_before_execution` covers empty, short, and long vectors. | H implementation helper; G remains for native bounds and explicit guest error handling. |
| H07 diagnostics / error traits | Host `Display`, Rust error traits, and generated trait helpers expose diagnostics/comparisons; they do not define target status words. | CLI invalid-argument test; typed constructor, shape, and arithmetic assertions. Individual wording is not a target contract. | H tooling only; G: guest status representation, text, and output device contract. |
| H08 CLI demo | Host arguments select a fixed synthetic fixture; compare H04/H05 and known output; print result and return process status. Unknown arguments fail with usage text. | `check_demo_succeeds`, `invalid_arguments_fail`; `cargo run -- --check`. | H tooling only; G: native boot, loader, program arguments, console and process exit. No guest service is silently supplied. |
| H09 pinned toolchain and CI | `rust-toolchain.toml` selects host Rust 1.99.1 with rustfmt and Clippy. `.github/workflows/ci.yml` checks formatting, denied-warning lints, full tests including rustdoc examples, and the synthetic CLI on an Ubuntu host. The repository token is read-only and checkout credentials are not persisted. | The same format/lint/test/demo commands pass locally. [GitHub CI run 2](https://github.com/Gyfted/TrinaryMatrix/actions/runs/40699582711) passes all four checks at commit `566657e`, including 9 integration tests and 1 doctest. Setup/download failure, command failure, or the ten-minute timeout fails the job; those workflow failure paths have not been fault-injected. | H development tooling only; toolchain downloads require network access, while the dependency-free crate commands use `--locked --offline`. No guest toolchain, boot, native execution, device behavior, or physical efficiency is established. |

All target capabilities remain G until a versioned native scope specifies their
contracts and proves their paths through encoded TrinaryASM execution, guest
routines, or a declared native device. Do not infer native capability from Rust
types, binary host arithmetic, host files, or the host test harness.

Before selecting an instruction, compare a proved sequence of existing operations
against alternatives, specify width/overflow/errors, and gather success/failure
and relevant fault evidence. Routine names do not justify dedicated opcodes.
No instruction set completeness, native platform completeness, or full guest
software-stack completeness is claimed. ISA v0.2 remains open.
