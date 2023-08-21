# Artisanally crafted, organic, locally grown wasifill example

## Pre-reqs

| Tool           | Description                                                                      |
| -------------- | -------------------------------------------------------------------------------- |
| [`just`][just] | A tool runner, similar to [`make`][make]                                         |
| [`wash`][wash] | CLI tool that powers [wasmCloud][wasmcloud], a WebAssembly compute mesh platform |

Along with the tools above, you'll need to set up a toolchain in one of the languages below along with tooling to build components:

| Language                  | Tooling                |
| ------------------------- | ---------------------- |
| [Rust][rust]              | [`wash`][wash]         |
| [Golang (TinyGo)][tinygo] | [`tinygo`][tinygo-bin] |

[just]: https://github.com/casey/just
[make]: https://www.gnu.org/software/make
[wasmcloud]: https://wasmcloud.com/
[wash]: https://github.com/wasmCloud/wash
[rust]: https://rust-lang.org
[tinygo]: https://tinygo.org/
[tinygo-bin]: https://tinygo.org/getting-started/install/

## Usage

```bash
just build
```

