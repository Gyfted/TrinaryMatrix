# Native-stack sequence

This repository currently contains a Rust host mathematical simulator. It is a
verification starting point, not an emulated native machine. Host `i8`, `i64`,
`i128`, `usize`, Rust memory layouts, files, and process I/O do not define target
representations or native services. The host reference must remain outside the
guest boundary. Keep the [coverage map](TRINARYASM_COVERAGE.md) current whenever
an API or target feature changes.

Follow these acceptance gates in order:

1. Define a versioned native machine contract: balanced trits/words, arithmetic
   and overflow, addresses, layouts, control, boot, memory and protection,
   exceptions, and explicit device protocols. Compare sequences of existing
   operations before proposing an instruction. Check canonical encodings and
   decoded/encoded execution against independent models and fault cases.
2. Implement compiler/runtime contracts: native types and calling rules,
   functions and control flow, executable loading, stack and memory services,
   explicit errors, and source-to-execution tests beyond inference.
3. Implement guest kernel ownership: native boot, protection, traps/interrupts,
   scheduling, and device services with defined recovery protocols.
4. Implement storage and I/O through native records, files, and explicit device
   contracts. Host backing is permissible only behind a specified ternary
   guest protocol; host filesystem calls are not guest filesystem capability.
5. Implement native libraries and applications, including checked inference.
   Each guest routine must have its own instruction/routine/device path and
   execution, conformance, resource-failure, and relevant fault evidence.
6. Run the development toolchain in the guest, compile/load programs there, and
   rebuild the toolchain to close the bootstrap loop.

ISA v0.2 stays open until evidence and review gates pass. Representing current
opcodes, establishing native platform capability, and completing the guest
software stack are separate claims. A correct host simulation establishes none
of the latter two, and it establishes no physical device efficiency.
