use bindings::exports::wasmcloud::messaging::handler::{BrokerMessage, Guest};

cargo_component_bindings::generate!();

struct Component {}

impl Guest for Component {
    /// handle subscription response
    fn handle_message(msg: BrokerMessage) -> Result<(), String> {
        // if the sender wants a reply
        if let Some(reply_to) = msg.reply_to {
            bindings::wasmcloud::messaging::consumer::publish(&BrokerMessage {
                body: msg.body,
                reply_to: None,
                subject: reply_to,
            })?;
        }
        Ok(())
    }
}
