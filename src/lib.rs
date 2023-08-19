::wit_bindgen::generate!({
    world: "wasifill",
    exports: {
        "wasmcloud:messaging/consumer": WasifillImpl,
        "wasmcloud:messaging-wasifill/guestcall-messaging": WasifillImpl,
    }
});

use crate::exports::wasmcloud::messaging::consumer;
use crate::wasmcloud::messaging::handler;

fn msg_to_export_msg(
    msg: handler::BrokerMessage,
) -> consumer::BrokerMessage {
    consumer::BrokerMessage {
        subject: msg.subject,
        body: msg.body,
        reply_to: msg.reply_to,
    }
}

fn export_msg_to_msg(
    msg: consumer::BrokerMessage,
) -> handler::BrokerMessage {
    handler::BrokerMessage {
        subject: msg.subject,
        body: msg.body,
        reply_to: msg.reply_to,
    }
}

// NOTE: I am only doing this for the types I need, but generated code should probably have all of
// the types enumerated from the wit definitions. Hopefully once
// https://github.com/bytecodealliance/wit-bindgen/issues/554 is resolved, we can remove this
// jankiness.

#[derive(::serde::Deserialize, ::serde::Serialize)]
#[serde(remote = "handler::BrokerMessage")]
struct BrokerMessage {
    subject: ::wit_bindgen::rt::string::String,
    body: Option<::wit_bindgen::rt::vec::Vec<u8>>,
    reply_to: Option<::wit_bindgen::rt::string::String>,
}

struct WasifillImpl;

impl exports::wasmcloud::messaging_wasifill::guestcall_messaging::GuestcallMessaging
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

// These are the custom types that should be identical to the ones that get generated for the provider stuff

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
struct RequestBody {
    subject: String,
    body: Option<Vec<u8>>,
    timeout_ms: u32,
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
struct RequestMultiBody {
    subject: String,
    body: Option<Vec<u8>>,
    timeout_ms: u32,
    max_results: u32,
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
struct PublishBody {
    #[serde(with = "BrokerMessage")]
    msg: handler::BrokerMessage,
}

impl exports::wasmcloud::messaging::consumer::Consumer for WasifillImpl {
    fn request(
        subject: ::wit_bindgen::rt::string::String,
        body: Option<::wit_bindgen::rt::vec::Vec<u8>>,
        timeout_ms: u32,
    ) -> Result<
        consumer::BrokerMessage,
        ::wit_bindgen::rt::string::String,
    > {
        // Take all the parameters and serialize them to the opaque payload we need to send
        let body = ::rmp_serde::to_vec_named(&RequestBody {
            subject,
            body,
            timeout_ms,
        })
        .map_err(|e| e.to_string())?;

        // Use the host call function to send the body
        let ret_data = wasmcloud::bus::host::call_sync(None, "Messaging.Request", &body)?;

        let mut de = ::rmp_serde::Deserializer::new(::std::io::Cursor::new(ret_data));
        // Get the response data back from the host call
        BrokerMessage::deserialize(&mut de)
            .map_err(|e| e.to_string())
            .map(msg_to_export_msg)
    }

    fn request_multi(
        subject: ::wit_bindgen::rt::string::String,
        body: Option<::wit_bindgen::rt::vec::Vec<u8>>,
        timeout_ms: u32,
        max_results: u32,
    ) -> Result<
        ::wit_bindgen::rt::vec::Vec<consumer::BrokerMessage>,
        ::wit_bindgen::rt::string::String,
    > {
        let body = ::rmp_serde::to_vec_named(&RequestMultiBody {
            subject,
            body,
            timeout_ms,
            max_results,
        })
        .map_err(|e| e.to_string())?;

        let ret_data = wasmcloud::bus::host::call_sync(None, "Messaging.RequestMulti", &body)?;

        // Ugly hack to get around remote derive of type
        #[derive(::serde::Deserialize)]
        struct Wrapper(#[serde(with = "BrokerMessage")] consumer::BrokerMessage);

        let mut de = ::rmp_serde::Deserializer::new(::std::io::Cursor::new(ret_data));
        <Vec<_> as ::serde::Deserialize>::deserialize(&mut de)
            .map(|v| {
                v.into_iter()
                    .map(|Wrapper(m)| msg_to_export_msg(m))
                    .collect()
            })
            .map_err(|e| e.to_string())
    }

    fn publish(
        msg: consumer::BrokerMessage,
    ) -> Result<(), ::wit_bindgen::rt::string::String> {
        let body = ::rmp_serde::to_vec_named(&PublishBody {
            msg: export_msg_to_msg(msg),
        })
        .map_err(|e| e.to_string())?;

        // Obviously I could do this differently, but for generated code it'll generate the same and then return nothing for a unit type return
        let _ret_data = wasmcloud::bus::host::call_sync(None, "Messaging.Publish", &body)?;

        Ok(())
    }
}
