This repo demonstrates a way to organize packages that whose purpose is to
generate Rust code. Typically this will entail an external input that is then
processed and transformed. Packages that do this often choose to expose this
code generation via a `proc_macro` or via a builder for use in a `build.rs`
file. The former doesn't require the creation on a specific build step and so
may be simpler, while the latter produces code that's easier to examine
(without resorting to `cargo expands` for example).

The constructs here are designed to let a single codegen implementation be used
either from a macro or from `build.rs` (or some other tool to produce file
artifacts with the generated code).

There are three crates:
- `codegen-impl` is where the real work happens. This is where code generation
actually happens.
- `codegen-macro` exports a `proc_macro`. It depends on `codegen-impl` and
invokes the functions exposed by it to do the code generation. Note that
packages may either define macros or other items to export so this package is
necessary for the subsequent package to export both macros and other items.
- `codegen` is the front-end package intended for direct use by consumrs. It
exposes both the macros from `codegen-macro` as well as the functionality from
`codegen-impl` (either directly or wrapped in APIs better suited for its
intended use).

To demonstrage the use of these, check out
- [`example-build`](./example-build) which uses a `build.rs` file to generate code, or...
- [`example-macro`](./example-macro) which generates code via the macros.

