# Changelog

All notable changes are documented in this file.
Lines marked “(!)” indicate a breaking change.

## [0.36.0](https://github.com/TritonVM/triton-vm/compare/v0.35.0..v0.36.0) - 2023-12-22

### ✨ Features

- Add benchmark for execution tracing ([11b360d6](https://github.com/TritonVM/triton-vm/commit/11b360d6))
- Record opstack underflow read/write in AET ([a57ef7c3](https://github.com/TritonVM/triton-vm/commit/a57ef7c3))
- Make Op Stack Table variable length ([b606dc60](https://github.com/TritonVM/triton-vm/commit/b606dc60))
- (!) Instruction `hash` only puts digest on stack ([2e37fb2f](https://github.com/TritonVM/triton-vm/commit/2e37fb2f))
- (!) Make instruction `pop` take an argument in range 1..=5 ([81248b90](https://github.com/TritonVM/triton-vm/commit/81248b90))
- (!) Make instruction `divine` take an argument in range 1..=5 ([5bf3541a](https://github.com/TritonVM/triton-vm/commit/5bf3541a))
- (!) Instruction `divine_sibling` pushes divined digest onto stack ([4602fad8](https://github.com/TritonVM/triton-vm/commit/4602fad8))
- Sponge instructions change stack size ([0fac3fc8](https://github.com/TritonVM/triton-vm/commit/0fac3fc8))
- Extension field instructions change stack size ([f0b3ab8f](https://github.com/TritonVM/triton-vm/commit/f0b3ab8f))
- (!) Make instruction `read_io` take an argument in range 1..=5 ([e138f0a0](https://github.com/TritonVM/triton-vm/commit/e138f0a0))
- (!) Make instruction `write_io` take an argument in range 1..=5 ([b8e5f978](https://github.com/TritonVM/triton-vm/commit/b8e5f978))
- Instruction `assert_vector` shrinks stack by 5 elements ([6a0e19cc](https://github.com/TritonVM/triton-vm/commit/6a0e19cc))
- (!) Make memory instructions take an argument in range 1..=5 ([8ef132af](https://github.com/TritonVM/triton-vm/commit/8ef132af))
- Add benchmark just executing a Triton VM program ([8301d5db](https://github.com/TritonVM/triton-vm/commit/8301d5db))
- (!) Improve error reporting ([48ee1099](https://github.com/TritonVM/triton-vm/commit/48ee1099))
- Only change VM state if instruction execution will work ([d7fbb3fd](https://github.com/TritonVM/triton-vm/commit/d7fbb3fd))
- Add `triton-tui`, a TUI for debugging programs in Triton assembly ([d0d79bce](https://github.com/TritonVM/triton-vm/commit/d0d79bce))
- Allow installing triton-tui as a binary ([047bed9b](https://github.com/TritonVM/triton-vm/commit/047bed9b))
- (de)serialize `VMState` ([8df0723c](https://github.com/TritonVM/triton-vm/commit/8df0723c))

### 🐛 Bug Fixes

- Crash VM when executing `swap 0` ([215f2ede](https://github.com/TritonVM/triton-vm/commit/215f2ede))
- Overflowing subtractions when accessing op stack underflow ([2aa72e77](https://github.com/TritonVM/triton-vm/commit/2aa72e77))
- *(doc)* Correct explanations for previous designs ([4bbc2d2a](https://github.com/TritonVM/triton-vm/commit/4bbc2d2a))
- Account for op stack table length dominating the AET ([f465f756](https://github.com/TritonVM/triton-vm/commit/f465f756))
- Correct calculation of total available memory in Triton VM ([18af2b40](https://github.com/TritonVM/triton-vm/commit/18af2b40))
- Fail Sponge instructions if Sponge state is uninitialized ([881b6c0d](https://github.com/TritonVM/triton-vm/commit/881b6c0d))

### ⚡️ Performance

- Remove redundant constraint preventing op stack underflow ([6215c108](https://github.com/TritonVM/triton-vm/commit/6215c108))
- Use instruction's fast-fail for error reporting, not cloning ([08bbc41f](https://github.com/TritonVM/triton-vm/commit/08bbc41f))

### 📚 Documentation

- Add TIP-0008 “Continuations” ([4b38d01b](https://github.com/TritonVM/triton-vm/commit/4b38d01b))
- Consistently use a space in “op stack” and “jump stack” ([eb8dc840](https://github.com/TritonVM/triton-vm/commit/eb8dc840))
- Delete out-of-date cheat sheet ([69aac2dc](https://github.com/TritonVM/triton-vm/commit/69aac2dc))
- Prose and example for Op Stack Table behavior ([db01232f](https://github.com/TritonVM/triton-vm/commit/db01232f))
- Update AET relations diagram ([f177d658](https://github.com/TritonVM/triton-vm/commit/f177d658))
- Op Stack Table padding ([ad09b8d2](https://github.com/TritonVM/triton-vm/commit/ad09b8d2))
- Update Op Stack Table's AIR ([3fb003b6](https://github.com/TritonVM/triton-vm/commit/3fb003b6))
- Update Processor Table's AET and AIR ([e59eedeb](https://github.com/TritonVM/triton-vm/commit/e59eedeb))
- Reflect changes to instructions, constraints, and mechanics ([ccf123b8](https://github.com/TritonVM/triton-vm/commit/ccf123b8))
- Exemplify error handling ([90151d6c](https://github.com/TritonVM/triton-vm/commit/90151d6c))
- Add changelog ([4d1fc2c0](https://github.com/TritonVM/triton-vm/commit/4d1fc2c0))

### ⚙️ Miscellaneous

- Simplify `use`s ([51878fae](https://github.com/TritonVM/triton-vm/commit/51878fae))
- *(test)* Remove unnecessary paths ([4323b202](https://github.com/TritonVM/triton-vm/commit/4323b202))
- `read_mem` starts reading at current address ([7faad183](https://github.com/TritonVM/triton-vm/commit/7faad183))
- (!) Rename & change debugging methods of `Program` ([abd17904](https://github.com/TritonVM/triton-vm/commit/abd17904))
- Fix spelling of `collinear` (not `colinear`) ([2e9ebd7c](https://github.com/TritonVM/triton-vm/commit/2e9ebd7c))
- Improve changelog generation configuration ([9e3432f3](https://github.com/TritonVM/triton-vm/commit/9e3432f3))
- (!) Remove `Default` derivation from `Program` ([868f49d9](https://github.com/TritonVM/triton-vm/commit/868f49d9))
- Allow tracing program execution from a given starting state ([5f702d47](https://github.com/TritonVM/triton-vm/commit/5f702d47))
- Upgrade dependency `cargo-tarpaulin` ([560f2555](https://github.com/TritonVM/triton-vm/commit/560f2555))

### ♻️ Refactor

- *(examples)* Return program, not instructions ([55c731ed](https://github.com/TritonVM/triton-vm/commit/55c731ed))
- Improve API of `VMProfiler` ([202cb74b](https://github.com/TritonVM/triton-vm/commit/202cb74b))
- *(vm)* Rename `ramp` to `ram_pointer` ([612714d0](https://github.com/TritonVM/triton-vm/commit/612714d0))
- *(processor_table)* Remove never-triggered panics ([6ced006a](https://github.com/TritonVM/triton-vm/commit/6ced006a))
- *(processor_table)* Remove unused struct `ExtProcessorTraceRow` ([d39230f2](https://github.com/TritonVM/triton-vm/commit/d39230f2))
- *(test)* Use crate `test-strategy` ([01e5e229](https://github.com/TritonVM/triton-vm/commit/01e5e229))
- *(test)* Improve testing instruction's transition constraints ([77948e1a](https://github.com/TritonVM/triton-vm/commit/77948e1a))
- *(op_stack)* Simplify recording of op stack underflow I/O calls ([f3803676](https://github.com/TritonVM/triton-vm/commit/f3803676))
- Turn python script for computing opcodes into a rust test ([ddb220f2](https://github.com/TritonVM/triton-vm/commit/ddb220f2))
- *(test)* Also test transition constraints on extension table ([4bd9cf16](https://github.com/TritonVM/triton-vm/commit/4bd9cf16))
- *(test)* Split test program enumeration into individual tests ([cc79cfad](https://github.com/TritonVM/triton-vm/commit/cc79cfad))
- Abstract over legal argument range for various instructions ([a76097e9](https://github.com/TritonVM/triton-vm/commit/a76097e9))
- (!) On success, `Stark::verify` returns `Ok(())`, not `Ok(true)` ([9d3a7065](https://github.com/TritonVM/triton-vm/commit/9d3a7065))
- (!) Remove `terminal_state`, allow running a VM state instead ([fbd58f1c](https://github.com/TritonVM/triton-vm/commit/fbd58f1c))
- Simplify indexing into `OpStack` ([4b31b2fe](https://github.com/TritonVM/triton-vm/commit/4b31b2fe))

### ✅ Testing

- Op stack table row sorting ([7418502b](https://github.com/TritonVM/triton-vm/commit/7418502b))
- Factor for running product with Op Stack Table never panics ([224e7923](https://github.com/TritonVM/triton-vm/commit/224e7923))
- Turn extension field instruction tests into property tests ([067d0053](https://github.com/TritonVM/triton-vm/commit/067d0053))
- Turn `get_colinear_y` into a property test ([39bd4668](https://github.com/TritonVM/triton-vm/commit/39bd4668))
- Use `proptest`, not ad-hoc prop tests, for program parsing tests ([d2acbbf8](https://github.com/TritonVM/triton-vm/commit/d2acbbf8))
- Delete some ignored, obsolete tests ([8deb268a](https://github.com/TritonVM/triton-vm/commit/8deb268a))
- Instructions fail before they modify the state ([c680fab2](https://github.com/TritonVM/triton-vm/commit/c680fab2))

## [0.35.0](https://github.com/TritonVM/triton-vm/compare/v0.34.1..v0.35.0) – 2023-10-17

### ✨ Features

- Better error reporting for failing `assert_vector` ([ee83ab6d](https://github.com/TritonVM/triton-vm/commit/ee83ab6d))
- Include debug information when printing `Program` ([d11aa541](https://github.com/TritonVM/triton-vm/commit/d11aa541))
- (!) Replace instruction `absorb_init` with `sponge_init` ([aca87471](https://github.com/TritonVM/triton-vm/commit/aca87471))
- (!) Add debug instruction `break` ([df6dc4b5](https://github.com/TritonVM/triton-vm/commit/df6dc4b5))

### 🐛 Bug Fixes

- (!) Use `Copy`-trait of `StarkParameters` ([70f3d957](https://github.com/TritonVM/triton-vm/commit/70f3d957))
- Linter v1.73.0 warnings ([641ed393](https://github.com/TritonVM/triton-vm/commit/641ed393))
- Print all helper variables and Sponge state of VM state ([07a54f6e](https://github.com/TritonVM/triton-vm/commit/07a54f6e))
- Disallow changing argument of `swap` to 0 ([bcf61ee6](https://github.com/TritonVM/triton-vm/commit/bcf61ee6))

### 📚 Documentation

- Adapt constraints to new instruction `sponge_init` ([cde031f0](https://github.com/TritonVM/triton-vm/commit/cde031f0))
- `Program` and its new methods ([5ff137dc](https://github.com/TritonVM/triton-vm/commit/5ff137dc))
- Align specification to code ([86a501ea](https://github.com/TritonVM/triton-vm/commit/86a501ea))
- Delete cheatsheet ([e8a5b526](https://github.com/TritonVM/triton-vm/commit/e8a5b526))

### ⚙️ Miscellaneous

- (!) Rename instruction `div` to `div_mod` ([c3ad923a](https://github.com/TritonVM/triton-vm/commit/c3ad923a))
- Reduce number of `as` castings ([540cf66f](https://github.com/TritonVM/triton-vm/commit/540cf66f))
- Remove unused `impl Display for InstructionToken` ([1550f8de](https://github.com/TritonVM/triton-vm/commit/1550f8de))
- Ignore JetBrains IDE's config files ([ddf9e7ad](https://github.com/TritonVM/triton-vm/commit/ddf9e7ad))

### ♻️ Refactor

- (!) Move `padded_height()` into `AET` ([f88d94f3](https://github.com/TritonVM/triton-vm/commit/f88d94f3))
- Store debug information `address_to_label` in `Program` ([d857e838](https://github.com/TritonVM/triton-vm/commit/d857e838))
- Improve readability of `Program` decoding ([e3741d68](https://github.com/TritonVM/triton-vm/commit/e3741d68))
- Extract VM profiling logic into `VMProfiler` ([97ecef8d](https://github.com/TritonVM/triton-vm/commit/97ecef8d))

### ✅ Testing

- Lower number of test cases for some FRI tests ([0159a688](https://github.com/TritonVM/triton-vm/commit/0159a688))
- Print error of failing `assert_vector` ([9bcdac2d](https://github.com/TritonVM/triton-vm/commit/9bcdac2d))
- Separate success and failure of `ensure_eq` ([86b97993](https://github.com/TritonVM/triton-vm/commit/86b97993))
- Improve property testing of Sponge instructions ([e3d90b8e](https://github.com/TritonVM/triton-vm/commit/e3d90b8e))
- Edge case when last Hash Table row is `sponge_init` ([f5f64963](https://github.com/TritonVM/triton-vm/commit/f5f64963))
- Too many returns crash VM, not `VMProfiler` ([a109b890](https://github.com/TritonVM/triton-vm/commit/a109b890))

## [0.34.1](https://github.com/TritonVM/triton-vm/compare/v0.34.0..v0.34.1) – 2023-10-05

### ⚙️ Miscellaneous

- Remove dependency `strum_macros` ([0e53844e](https://github.com/TritonVM/triton-vm/commit/0e53844e))

## [0.34.0](https://github.com/TritonVM/triton-vm/compare/v0.33.0..v0.34.0) – 2023-10-05

### ✨ Features

- Add methods indicating instruction's effect on stack size ([2f3867d8](https://github.com/TritonVM/triton-vm/commit/2f3867d8))
- Derive `Default` for `NonDeterminism` ([954e23e3](https://github.com/TritonVM/triton-vm/commit/954e23e3))

### 📚 Documentation

- Add example uses of Triton VM to top-level documentation ([6c3537ab](https://github.com/TritonVM/triton-vm/commit/6c3537ab))
- Fix specification for instruction `pow` ([be168ef9](https://github.com/TritonVM/triton-vm/commit/be168ef9))

### ♻️ Refactor

- Refactor FRI for increased readability ([61073e2b](https://github.com/TritonVM/triton-vm/commit/61073e2b))
- (!) Don't expose profiling macros ([b2e2a600](https://github.com/TritonVM/triton-vm/commit/b2e2a600))
- Introduce profiling category for witness generation ([6362cc57](https://github.com/TritonVM/triton-vm/commit/6362cc57))

### ✅ Testing

- Use `Arbitrary` for property based tests ([74df64d5](https://github.com/TritonVM/triton-vm/commit/74df64d5))

## [0.33.0](https://github.com/TritonVM/triton-vm/compare/v0.32.1..v0.33.0) – 2023-08-10

### ✨ Features

- (!) Initialize Triton VM's RAM non-deterministically ([fb314aea](https://github.com/TritonVM/triton-vm/commit/fb314aea))

### ⚡️ Performance

- (!) Shrink FRI domain size by splitting quotients into segments ([c4f1e554](https://github.com/TritonVM/triton-vm/commit/c4f1e554))
- Parallelize random linear summing ([1b6b2d4a](https://github.com/TritonVM/triton-vm/commit/1b6b2d4a), [13dfb28f](https://github.com/TritonVM/triton-vm/commit/13dfb28f))
- Use uninitialized memory for some allocations ([b1724829](https://github.com/TritonVM/triton-vm/commit/b1724829))

### 📚 Documentation

- Add specification for TIP-0007 – Run-Time Permutation Check ([82f81f36](https://github.com/TritonVM/triton-vm/commit/82f81f36))

### ⚙️ Miscellaneous

- Upgrade dependencies ([7f24c6eb](https://github.com/TritonVM/triton-vm/commit/7f24c6eb))

### ♻️ Refactor

- Slightly improve interface for `Arithmetic Domain` ([c9907e24](https://github.com/TritonVM/triton-vm/commit/c9907e24))
- Use `.borrow()` directly instead of on explicit `.as_ref()` ([07af1875](https://github.com/TritonVM/triton-vm/commit/07af1875))
- Derive trait `GetSize` for `Proof` instead of implementing it manually ([6ffa9e3d](https://github.com/TritonVM/triton-vm/commit/6ffa9e3d))
- Simplify indexing of `Challenges` through `Index` trait ([bbbc52c8](https://github.com/TritonVM/triton-vm/commit/bbbc52c8))

### ✅ Testing

- Add various tests ([18e81ffd](https://github.com/TritonVM/triton-vm/commit/18e81ffd), [21bb1ae5](https://github.com/TritonVM/triton-vm/commit/21bb1ae5), [364a4464](https://github.com/TritonVM/triton-vm/commit/364a4464))

## [0.32.1](https://github.com/TritonVM/triton-vm/compare/v0.32.0..v0.32.1) – 2023-08-01

### ⚙️ Miscellaneous

- Upgrade dependencies ([5683ea78](https://github.com/TritonVM/triton-vm/commit/5683ea78))

## [0.32.0](https://github.com/TritonVM/triton-vm/compare/v0.31.1..v0.32.0) – 2023-08-01

### ✨ Features

- Introduce macros for writing Triton assembly: `triton_asm!` and `triton_program!`

### ⚙️ Miscellaneous

- Profile runs of the VM

### ♻️ Refactor

- Merge crates `triton-opcodes` and `triton-profiler` into `triton-vm`
- (!) Rename `simulate` to `trace_execution`

## [0.31.1](https://github.com/TritonVM/triton-vm/compare/v0.31.0..v0.31.1) – 2023-07-06

### ✨ Features

- Add helper method for program hashing ([b0555e28](https://github.com/TritonVM/triton-vm/commit/b0555e28))

### ⚙️ Miscellaneous

- Use triton-opcodes v0.31.1 ([59f2c103](https://github.com/TritonVM/triton-vm/commit/59f2c103))

## [0.31.0](https://github.com/TritonVM/triton-vm/compare/v0.30.0..v0.31.0) – 2023-07-05

### 🐛 Bug Fixes

- (!) Don't include `Claim` in `Proof` ([741c6d8f](https://github.com/TritonVM/triton-vm/commit/741c6d8f))

## [0.30.0](https://github.com/TritonVM/triton-vm/compare/v0.29.1..v0.30.0) – 2023-07-04

### ✨ Features

- (!) Attest to the program being run ([5c42531f](https://github.com/TritonVM/triton-vm/commit/5c42531f))
- Add debugging function consuming considerably less RAM ([e556535c](https://github.com/TritonVM/triton-vm/commit/e556535c))

### 🐛 Bug Fixes

- (!) Fix soundness bugs in `skiz` ([a697f3f8](https://github.com/TritonVM/triton-vm/commit/a697f3f8))

### ⚙️ Miscellaneous

- Add benchmarks for proof size ([b4517f85](https://github.com/TritonVM/triton-vm/commit/b4517f85), [618d9443](https://github.com/TritonVM/triton-vm/commit/618d9443))

## [0.29.1](https://github.com/TritonVM/triton-vm/compare/v0.29.0..v0.29.1) – 2023-06-19

### 🐛 Bug Fixes

- Correct opcodes of stack-shrinking u32 instructions ([3d9f838c](https://github.com/TritonVM/triton-vm/commit/3d9f838c))

## [0.29.0](https://github.com/TritonVM/triton-vm/compare/v0.28.0..v0.29.0) – 2023-06-15

### 🐛 Bug Fixes

- executing `lt` on operands 0 and 0 is now possible ([10360b11](https://github.com/TritonVM/triton-vm/commit/10360b11))

### ♻️ Refactor

- (!) `vm::simulate()` returns `Result<_>` ([f52e4a90](https://github.com/TritonVM/triton-vm/commit/f52e4a90))

## [0.28.0](https://github.com/TritonVM/triton-vm/compare/v0.25.1..v0.28.0) – 2023-06-13

### ✨ Features

- Add native interface for proving `Claim`s ([4f2f02ff](https://github.com/TritonVM/triton-vm/commit/4f2f02ff))

### 🐛 Bug Fixes

- (!) Include `Claim` in Fiat-Shamir heuristic ([c786c915](https://github.com/TritonVM/triton-vm/commit/c786c915))

### ⚡️ Performance

- Lower the AIR degree to 4 ([73f9e0a0](https://github.com/TritonVM/triton-vm/commit/73f9e0a0))

### ♻️ Refactor

- Derive `BFieldCodec` where possible ([dc528c41](https://github.com/TritonVM/triton-vm/commit/dc528c41))
- Derive `padded_height` from `Proof` ([0d6c1811](https://github.com/TritonVM/triton-vm/commit/0d6c1811))
- Remove the `padded_height` from the `Claim` ([4be177eb](https://github.com/TritonVM/triton-vm/commit/4be177eb))
- (!) Remove field `Uncast` from `ProofItem` ([27461b10](https://github.com/TritonVM/triton-vm/commit/27461b10))
- (!) Remove trait `MayBeUncast` ([27461b10](https://github.com/TritonVM/triton-vm/commit/27461b10))

### ✅ Testing

- Remove `TestItem`, test using actual `ProofItem`s instead ([54afb081](https://github.com/TritonVM/triton-vm/commit/54afb081))

## [0.25.1](https://github.com/TritonVM/triton-vm/compare/v0.24.0..v0.25.1) – 2023-05-24

### ⚙️ Miscellaneous

- Upgrade dependencies ([328685fb](https://github.com/TritonVM/triton-vm/commit/328685fb))

## [0.24.0](https://github.com/TritonVM/triton-vm/compare/v0.21.0..v0.24.0) – 2023-05-23

### ✨ Features

- Add `BFieldCodec` functions for `Program`, `Claim`, and `Proof` ([07f09801](https://github.com/TritonVM/triton-vm/commit/07f09801), [5e3f66be](https://github.com/TritonVM/triton-vm/commit/5e3f66be))
- Get size for `Program` and `Claim` ([2d827e9c](https://github.com/TritonVM/triton-vm/commit/2d827e9c), [77eed359](https://github.com/TritonVM/triton-vm/commit/77eed359))

### ♻️ Refactor

- (!) Move `BFieldCodec` trait and implementations to `twenty-first` ([193ffa3e](https://github.com/TritonVM/triton-vm/commit/193ffa3e))

## [0.21.0](https://github.com/TritonVM/triton-vm/compare/v0.20.0..v0.21.0) – 2023-05-09

### 🐛 Bug Fixes

- Correct number of checks performed between DEEP-ALI and FRI ([0400d0c3](https://github.com/TritonVM/triton-vm/commit/0400d0c3))

### ⚡️ Performance

- Avoid unnecessary hashing in the Fiat-Shamir heuristic ([0bc5f63d](https://github.com/TritonVM/triton-vm/commit/0bc5f63d))
- Special-case base and extension fields when evaluating the AIR ([4547b961](https://github.com/TritonVM/triton-vm/commit/4547b961))

### 📚 Documentation

- Document zk-STARK parameters ([b1740b71](https://github.com/TritonVM/triton-vm/commit/b1740b71))
- Lower the number of trace randomizers to match DEEP-ALI ([b1740b71](https://github.com/TritonVM/triton-vm/commit/b1740b71))

### ⚙️ Miscellaneous

- Upgrade dependency `twenty-first` ([f0ab8c0a](https://github.com/TritonVM/triton-vm/commit/f0ab8c0a))

### ♻️ Refactor

- (!) Remove state from the STARK's struct ([9d92e0db](https://github.com/TritonVM/triton-vm/commit/9d92e0db))
- Drop the claimed padded height from the proof ([2372461a](https://github.com/TritonVM/triton-vm/commit/2372461a))
- Sort profiled categories by their duration ([57825878](https://github.com/TritonVM/triton-vm/commit/57825878))
- Fail with specific error message instead of panic ([7cf318be](https://github.com/TritonVM/triton-vm/commit/7cf318be))
- Make `StarkParameters` (de)serializeable ([ca7cbe03](https://github.com/TritonVM/triton-vm/commit/ca7cbe03))
- Don't rely on `Digest`s being `Hashable` ([d504fc20](https://github.com/TritonVM/triton-vm/commit/d504fc20))

### ✅ Testing

- Add test for the DEEP update ([29780b06](https://github.com/TritonVM/triton-vm/commit/29780b06))
- Make some assertions already at compile-time ([564a1279](https://github.com/TritonVM/triton-vm/commit/564a1279), [e12db597](https://github.com/TritonVM/triton-vm/commit/e12db597))
- When testing the STARK, use `2^log_2_exp_factor`, not `2^(2^log_2_exp_factor)` ([304a7ea7](https://github.com/TritonVM/triton-vm/commit/304a7ea7))

## [0.20.0](https://github.com/TritonVM/triton-vm/compare/v0.19.0..v0.20.0) – 2023-04-24

### ✨ Features

- (!) Do DEEP-ALI instead of plain ALI in the zk-STARK ([96064413](https://github.com/TritonVM/triton-vm/commit/96064413))
- Add convenience functions for using Triton VM ([0dab32a2](https://github.com/TritonVM/triton-vm/commit/0dab32a2)) ([dda05e4e](https://github.com/TritonVM/triton-vm/commit/dda05e4e))
- Improve Triton profiler ([7edd1a2c](https://github.com/TritonVM/triton-vm/commit/7edd1a2c))
- Make method `debug` more powerful ([ab49df75](https://github.com/TritonVM/triton-vm/commit/ab49df75))

### ⚙️ Miscellaneous

- Add construction of AET (witness generation) to profiling ([c6e7b1e1](https://github.com/TritonVM/triton-vm/commit/c6e7b1e1))

### ♻️ Refactor

- Use `cfg(debug_assertions)`, not environment variable ([b0052f1f](https://github.com/TritonVM/triton-vm/commit/b0052f1f))

## [0.19.0](https://github.com/TritonVM/triton-vm/compare/v0.18.0..v0.19.0) – 2023-03-17

### ✨ Features

- add instruction `pop_count` ([efd90c65](https://github.com/TritonVM/triton-vm/commit/efd90c65))

### ♻️ Refactor

- (!) Parse instructions `dup` and `swap` as taking arguments ([4eecac2b](https://github.com/TritonVM/triton-vm/commit/4eecac2b))
- (!) Enforce labels to start with an alphabetic character or `_` ([5a5e6bad](https://github.com/TritonVM/triton-vm/commit/5a5e6bad))
- (!) Remove method `simulate_no_input` ([089af774](https://github.com/TritonVM/triton-vm/commit/089af774))
- (!) Rename `run` to `debug`, introduce new `run` without debug capabilities ([8bd880ff](https://github.com/TritonVM/triton-vm/commit/8bd880ff))

## [0.18.0](https://github.com/TritonVM/triton-vm/compare/v0.14.0..v0.18.0) – 2023-03-10

### ✨ Features

- (!) Change behavior of instructions `read_mem` and `write_mem` ([022245b7](https://github.com/TritonVM/triton-vm/commit/022245b7))
- (!) Move to Tip5 hash function ([d40f0b62](https://github.com/TritonVM/triton-vm/commit/d40f0b62))
- Use `nom` for parsing Triton assembly ([bbe4aa87](https://github.com/TritonVM/triton-vm/commit/bbe4aa87), [8602892f](https://github.com/TritonVM/triton-vm/commit/8602892f))

### ⚡️ Performance

- Improve constant folding in multicircuits ([c1be5bb9](https://github.com/TritonVM/triton-vm/commit/c1be5bb9))

### 📚 Documentation

- Explain the various cross-table arguments ([567efc00](https://github.com/TritonVM/triton-vm/commit/567efc00))
- Add and improve intra-document links ([a66b20dd](https://github.com/TritonVM/triton-vm/commit/a66b20dd), [9386878c](https://github.com/TritonVM/triton-vm/commit/9386878c))

### ⚙️ Miscellaneous

- Upgrade dependencies ([ff972ff8](https://github.com/TritonVM/triton-vm/commit/ff972ff8))

### ♻️ Refactor

- Replace Instruction Table by Lookup Argument ([543327a0](https://github.com/TritonVM/triton-vm/commit/543327a0))
- Rework U32 Table ([09a4c277](https://github.com/TritonVM/triton-vm/commit/09a4c277), [27190307](https://github.com/TritonVM/triton-vm/commit/27190307))
- Improve clock jump differences check ([04bb5c48](https://github.com/TritonVM/triton-vm/commit/04bb5c48), [9c2f3c0b](https://github.com/TritonVM/triton-vm/commit/9c2f3c0b))

## [0.14.0](https://github.com/TritonVM/triton-vm/compare/v0.13.0..v0.14.0) – 2023-01-20

### ✨ Features

- (!) Introduce Sponge instructions `absorb_init`, `absorb`, and `squeeze` ([af6a9e0e](https://github.com/TritonVM/triton-vm/commit/af6a9e0e))
- Add `nom` parser for Triton Assembly ([ed9e4a90](https://github.com/TritonVM/triton-vm/commit/ed9e4a90))

## [0.13.0](https://github.com/TritonVM/triton-vm/compare/v0.11.0..v0.13.0) – 2023-01-12

### ✨ Features

- (!) Add u32 instructions ([1f3eae84](https://github.com/TritonVM/triton-vm/commit/1f3eae84))

### 📚 Documentation

- Add TIP-0006: Program Attestation ([c694b4c5](https://github.com/TritonVM/triton-vm/commit/c694b4c5))

## [0.11.0](https://github.com/TritonVM/triton-vm/compare/v0.10.0..v0.11.0) – 2022-12-22

### 🐛 Bug Fixes

- (!) Enforce RAM initialization to all zero ([#155](https://github.com/TritonVM/triton-vm/issues/155))

### ⚙️ Miscellaneous

- Upgrade dependencies ([79189cb8](https://github.com/TritonVM/triton-vm/commit/79189cb8))

### ♻️ Refactor

- Represent AET as consecutive memory region ([4477d758](https://github.com/TritonVM/triton-vm/commit/4477d758))
- Distinguish AIR constraints of base and extension tables ([#119](https://github.com/TritonVM/triton-vm/issues/119))
- Reduce memory footprint ([#11](https://github.com/TritonVM/triton-vm/issues/11))
- Split Triton VM's instructions into separate sub-crate ([7bcc09ea](https://github.com/TritonVM/triton-vm/commit/7bcc09ea))

## [0.10.0](https://github.com/TritonVM/triton-vm/compare/v0.9.0..v0.10.0) – 2022-12-19

### 🐛 Bug Fixes

- (!) Adjust `::sample_weights()` and `::sample_indices()` ([cfb0fcb6](https://github.com/TritonVM/triton-vm/commit/cfb0fcb6))

## [0.9.0](https://github.com/TritonVM/triton-vm/compare/v0.8.0..v0.9.0) – 2022-12-08

### ✨ Features

- (!) Allow reading from uninitialized memory, returning zero ([444bb973](https://github.com/TritonVM/triton-vm/commit/444bb973))

## [0.8.0](https://github.com/TritonVM/triton-vm/compare/v0.7.0..v0.8.0) – 2022-12-08

### ✨ Features

- Allow comments in tasm code ([cdbcf439](https://github.com/TritonVM/triton-vm/commit/cdbcf439))

### 🐛 Bug Fixes

- Fail on duplicate labels in parser ([42c41ac2](https://github.com/TritonVM/triton-vm/commit/42c41ac2))

### ⚡️ Performance

- Use iNTT, not fast-interpolate, for polynomial interpolation ([908b7c5f](https://github.com/TritonVM/triton-vm/commit/908b7c5f))

### ♻️ Refactor

- Derive quotient domain the right way around ([d0d3c4f1](https://github.com/TritonVM/triton-vm/commit/d0d3c4f1))
- Use compile-time constants for table's width ([c4868111](https://github.com/TritonVM/triton-vm/commit/c4868111))
- Remove type parameter from arithmetic domain ([381d3643](https://github.com/TritonVM/triton-vm/commit/381d3643))
- Always use `BFieldElements` to perform low-degree extension ([b873f503](https://github.com/TritonVM/triton-vm/commit/b873f503))

## [0.7.0](https://github.com/TritonVM/triton-vm/compare/v0.3.1..v0.7.0) – 2022-11-22

### 🐛 Bug Fixes

- correctly decode `Vec<PartialAuthenticationPath>` ([e7fd6cc2](https://github.com/TritonVM/triton-vm/commit/e7fd6cc2))

### ⚡️ Performance

- (!) Use rust code, not symbolic polynomials, for the AIR ([cd62c59c](https://github.com/TritonVM/triton-vm/commit/cd62c59c))
- Use quotient domain instead of FRI domain wherever applicable ([776fa19c](https://github.com/TritonVM/triton-vm/commit/776fa19c))
- Don't multiply randomizer codeword by random weight ([b105b68d](https://github.com/TritonVM/triton-vm/commit/b105b68d))

### 📚 Documentation

- Add TIP 0004: “Drop U32 Table” ([38293c4e](https://github.com/TritonVM/triton-vm/commit/38293c4e))

### ⚙️ Miscellaneous

- Upgrade dependencies ([cc15b183](https://github.com/TritonVM/triton-vm/commit/cc15b183))
- Add prove_fib_100 benchmark for STARK proving ([1326b12d](https://github.com/TritonVM/triton-vm/commit/1326b12d))

### ♻️ Refactor

- Replace `Result<T, Box<dyn Error>>` with `anyhow::Result<T>` ([448d4cdd](https://github.com/TritonVM/triton-vm/commit/448d4cdd))
- Run `cargo fmt` after constraint-evaluation-generator ([2d183e49](https://github.com/TritonVM/triton-vm/commit/2d183e49))
- Replace TimingReporter with TritonProfiler ([c40c1bc0](https://github.com/TritonVM/triton-vm/commit/c40c1bc0))
- Drop `VecStream` in favor of `Vec<BFieldElement>` ([9668fbea](https://github.com/TritonVM/triton-vm/commit/9668fbea))

## [0.3.1](https://github.com/TritonVM/triton-vm/compare/efae926a43e3b972659cf6d756f2457cd94e4f2e..v0.3.1) – 2022-10-20

Initial release of Triton VM.
