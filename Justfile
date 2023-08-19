just := env_var_or_default("JUST", just_executable())
cargo := env_var_or_default("CARGO", "cargo")
wasm_tools := env_var_or_default("WASM_TOOLS", "wasm-tools")

wasi_release_path := "target/wasm32-wasi/release"

_default:
    {{just}} --list

build-actor:
    @{{cargo}} build --manifest-path=actor/Cargo.toml --target=wasm32-wasi --release
    @{{wasm_tools}} component new --adapt=wasi_snapshot_preview1.wasm actor/{{wasi_release_path}}/messaging_wit_test.wasm -o actor.component.wasm

build-multiplexer:
    @{{cargo}} build --manifest-path=multiplexer/Cargo.toml --target=wasm32-wasi --release
    @{{wasm_tools}} component new --adapt=wasi_snapshot_preview1.wasm multiplexer/{{wasi_release_path}}/multiplexer.wasm -o multiplexer.component.wasm

build-wasifill:
    @{{cargo}} build --target=wasm32-wasi --release
    @{{wasm_tools}} component new --adapt=wasi_snapshot_preview1.wasm {{wasi_release_path}}/wasifill_example.wasm -o wasifill.component.wasm

# Build the WASM component
build: build-actor build-multiplexer build-wasifill 
   @echo "[success]"