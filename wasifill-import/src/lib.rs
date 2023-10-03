cargo_component_bindings::generate!({
    additional_derives: [serde::Serialize, serde::Deserialize],
});

use std::io::Write;

use serde::Serialize;

struct Component;

impl bindings::exports::wasmcloud::messaging_wasifill_import::guestcall_messaging::Guest
    for Component
{
    fn guestcall_messaging(operation: String) -> Result<(), String> {
        // Please note that here we only have one operation, but there could be multiple so this
        // should always generate a match statement
        let mut output = rmp_serde::Serializer::new(std::io::stdout()).with_struct_map();
        let res = match operation.as_ref() {
            "Message.Handle" => {
                let msg: bindings::wasmcloud::messaging::handler::BrokerMessage =
                    rmp_serde::from_read(std::io::stdin()).map_err(|e| e.to_string())?;

                bindings::wasmcloud::messaging::handler::handle_message(&msg)
                    .serialize(&mut output)
                    .map_err(|e| e.to_string())
            }
            _ => Err(format!("unknown operation {}", operation)),
        };
        output.into_inner().flush().map_err(|e| e.to_string())?;
        res
    }
}
