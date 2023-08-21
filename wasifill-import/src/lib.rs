::wit_bindgen::generate!({
    world: "wasifill-import",
    exports: {
        "wasmcloud:messaging-wasifill-import/guestcall-messaging": WasifillImpl,
    }
});

// NOTE: I am only doing this for the types I need, but generated code should probably have all of
// the types enumerated from the wit definitions. Hopefully once
// https://github.com/bytecodealliance/wit-bindgen/issues/554 is resolved, we can remove this
// jankiness.

#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(remote = "wasmcloud::messaging::types::BrokerMessage")]
struct BrokerMessage {
    subject: ::wit_bindgen::rt::string::String,
    body: Option<::wit_bindgen::rt::vec::Vec<u8>>,
    reply_to: Option<::wit_bindgen::rt::string::String>,
}

struct WasifillImpl;

impl exports::wasmcloud::messaging_wasifill_import::guestcall_messaging::GuestcallMessaging
    for WasifillImpl
{
    fn guestcall_messaging(
        operation: ::wit_bindgen::rt::string::String,
    ) -> Result<(), ::wit_bindgen::rt::string::String> {
        // Please note that here we only have one operation, but there could be multiple so this
        // should always generate a match statement
        match operation.as_ref() {
            "Message.Handle" => {
                let mut de = ::rmp_serde::Deserializer::new(std::io::stdin());
                let msg = BrokerMessage::deserialize(&mut de).map_err(|e| e.to_string())?;
                wasmcloud::messaging::handler::handle_message(&msg)
            }
            _ => Err(format!("unknown operation {}", operation)),
        }
    }
}
