cargo_component_bindings::generate!();

struct Component;

impl bindings::exports::wasmcloud::bus::guest::Guest for Component {
    fn call(operation: String) -> Result<(), String> {
        let prefix = match operation.split_once('.') {
            Some((prefix, _)) => prefix.to_lowercase(),
            None => return Err(format!("Unknown operation: {}", operation)),
        };

        match prefix.as_ref() {
            "message" => {
                bindings::wasmcloud::messaging_wasifill_import::guestcall_messaging::guestcall_messaging(
                    &operation,
                )
            }
            // For multiple contracts, other strings would go here
            _ => Err(format!("Unknown operation: {}", operation)),
        }
    }
}
