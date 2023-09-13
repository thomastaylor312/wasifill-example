use exports::wasmcloud::messaging::handler::{BrokerMessage, Guest};

wit_bindgen::generate!({
    world: "actor",
    exports: {
        "wasmcloud:messaging/handler": MessagingWitTestActor,
    }
});

struct MessagingWitTestActor {}

impl Guest for MessagingWitTestActor {
    /// handle subscription response
    fn handle_message(msg: BrokerMessage) -> Result<(), String> {
        // if the sender wants a reply
        if let Some(reply_to) = msg.reply_to {
            wasmcloud::messaging::consumer::publish(&BrokerMessage {
                body: msg.body,
                reply_to: None,
                subject: reply_to,
            })?;
        }
        Ok(())
    }
}
