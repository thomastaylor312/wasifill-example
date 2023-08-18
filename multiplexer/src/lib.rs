wit_bindgen::generate!({
    world: "multiplexer",
    exports: {
        "wasmcloud:bus/guest": MultiplexerImpl
    }
});

struct MultiplexerImpl;

impl exports::wasmcloud::bus::guest::Guest for MultiplexerImpl {
    fn call(
        operation: wit_bindgen::rt::string::String,
    ) -> Result<(), wit_bindgen::rt::string::String> {
        let prefix = match operation.split_once('.') {
            Some((prefix, _)) => prefix.to_lowercase(),
            None => return Err(format!("Unknown operation: {}", operation)),
        };

        match prefix.as_ref() {
            "messaging" => {
                wasmcloud::messaging_wasifill::guestcall_messaging::guestcall_messaging(&operation)
            }
            // For multiple contracts, other strings would go here
            _ => Err(format!("Unknown operation: {}", operation)),
        }
    }
}
