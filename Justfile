just := env_var_or_default("JUST", just_executable())
cargo := env_var_or_default("CARGO", "cargo")
wasm_tools := env_var_or_default("WASM_TOOLS", "wasm-tools")
wash := env_var_or_default("WASH", "wash")

wasi_release_path := "target/wasm32-wasi/release"

_default:
    {{just}} --list

build-actor:
    @{{cargo}} component build --manifest-path=actor/Cargo.toml --release 
    @cp actor/{{wasi_release_path}}/messaging_wit_test.wasm ./actor.component.wasm
    @{{cargo}} build --manifest-path=actor/Cargo.toml --target=wasm32-wasi --release
    @{{wasm_tools}} component new --adapt=wasi_snapshot_preview1.wasm actor/{{wasi_release_path}}/messaging_wit_test.wasm -o actor.component.wasm

build-multiplexer:
    @{{cargo}} component build --manifest-path=multiplexer/Cargo.toml --release 
    @cp multiplexer/{{wasi_release_path}}/multiplexer.wasm ./multiplexer.component.wasm
    @{{cargo}} build --manifest-path=multiplexer/Cargo.toml --target=wasm32-wasi --release
    @{{wasm_tools}} component new --adapt=wasi_snapshot_preview1.wasm multiplexer/{{wasi_release_path}}/multiplexer.wasm -o multiplexer.component.wasm

build-wasifills:
    @{{cargo}} component build --manifest-path=wasifill-export/Cargo.toml --release
    @{{cargo}} component build --manifest-path=wasifill-import/Cargo.toml --release
    @cp wasifill-export/{{wasi_release_path}}/wasifill_example_export.wasm ./wasifill_export.component.wasm
    @cp wasifill-import/{{wasi_release_path}}/wasifill_example_import.wasm ./wasifill_import.component.wasm

compose:
    @{{wasm_tools}} compose actor.component.wasm -d wasifill_export.component.wasm -o step1.wasm
    @{{wasm_tools}} compose wasifill_import.component.wasm -d step1.wasm -o step2.wasm
    @{{wasm_tools}} compose multiplexer.component.wasm -d step2.wasm -o composed.component.wasm 
    @rm step1.wasm step2.wasm

sign:
    @{{wash}} claims sign --name messaging_wit_test -c wasmcloud:messaging2 composed.component.wasm

# Build the WASM component
build: build-actor build-multiplexer build-wasifills compose
   @echo "[success] Signed actor component available at: composed.component_s.wasm"
