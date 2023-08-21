::wit_bindgen::generate!({
    world: "wasifill-export",
    exports: {
        "wasmcloud:messaging/consumer": WasifillImpl,
    }
});

// NOTE: We need this because the generated rust code uses a type alias `pub type BrokerMessage =
// path::to::common::message::type::BrokerMessage;`. This causes it to be a separate concrete type,
// so _every_ custom type will need these stupid conversion functions.

fn msg_to_export_msg(
    msg: wasmcloud::messaging::types::BrokerMessage,
) -> exports::wasmcloud::messaging::consumer::BrokerMessage {
    exports::wasmcloud::messaging::consumer::BrokerMessage {
        subject: msg.subject,
        body: msg.body,
        reply_to: msg.reply_to,
    }
}

fn export_msg_to_msg(
    msg: exports::wasmcloud::messaging::consumer::BrokerMessage,
) -> wasmcloud::messaging::types::BrokerMessage {
    wasmcloud::messaging::types::BrokerMessage {
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
#[serde(remote = "wasmcloud::messaging::types::BrokerMessage")]
struct BrokerMessage {
    subject: ::wit_bindgen::rt::string::String,
    body: Option<::wit_bindgen::rt::vec::Vec<u8>>,
    reply_to: Option<::wit_bindgen::rt::string::String>,
}

struct WasifillImpl;

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
    msg: wasmcloud::messaging::types::BrokerMessage,
}

impl exports::wasmcloud::messaging::consumer::Consumer for WasifillImpl {
    fn request(
        subject: ::wit_bindgen::rt::string::String,
        body: Option<::wit_bindgen::rt::vec::Vec<u8>>,
        timeout_ms: u32,
    ) -> Result<
        exports::wasmcloud::messaging::consumer::BrokerMessage,
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
        ::wit_bindgen::rt::vec::Vec<exports::wasmcloud::messaging::consumer::BrokerMessage>,
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
        struct Wrapper(#[serde(with = "BrokerMessage")] wasmcloud::messaging::types::BrokerMessage);

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
        msg: exports::wasmcloud::messaging::consumer::BrokerMessage,
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
