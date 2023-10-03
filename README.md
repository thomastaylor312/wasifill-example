# Artisanally crafted, organic, locally grown wasifill example

## Pre-reqs

You will need to have a [Rust][rust] toolchain installed in addition to the following tools:

| Tool                                 | Description                                                                      |
| ------------------------------------ | -------------------------------------------------------------------------------- |
| [`just`][just]                       | A tool runner, similar to [`make`][make]                                         |
| [`wash`][wash]                       | CLI tool that powers [wasmCloud][wasmcloud], a WebAssembly compute mesh platform |
| [`cargo component`][cargo-component] | A cargo plugin that is wasm component aware                                      |

[just]: https://github.com/casey/just
[make]: https://www.gnu.org/software/make
[wasmcloud]: https://wasmcloud.com/
[wash]: https://github.com/wasmCloud/wash
[rust]: https://rust-lang.org
[cargo-component]: https://github.com/bytecodealliance/cargo-component

## Usage

```bash
just build
```

