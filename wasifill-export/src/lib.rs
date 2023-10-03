cargo_component_bindings::generate!({
    additional_derives: [serde::Serialize, serde::Deserialize],
});

struct Component;

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
    msg: bindings::exports::wasmcloud::messaging::consumer::BrokerMessage,
}

impl bindings::exports::wasmcloud::messaging::consumer::Guest for Component {
    fn request(
        subject: String,
        body: Option<Vec<u8>>,
        timeout_ms: u32,
    ) -> Result<bindings::exports::wasmcloud::messaging::consumer::BrokerMessage, String> {
        // Take all the parameters and serialize them to the opaque payload we need to send
        let body = ::rmp_serde::to_vec_named(&RequestBody {
            subject,
            body,
            timeout_ms,
        })
        .expect("Error when serializing data to send to host");

        // Use the host call function to send the body
        let ret_data = bindings::wasmcloud::bus::host::call_sync(
            None,
            "wasmcloud:messaging2/Message.Request",
            &body,
        )
        .expect("Error when calling host");

        // Get the response data back from the host call
        rmp_serde::from_read(std::io::Cursor::new(ret_data))
            .expect("Unable to deserialize body from host")
    }

    fn request_multi(
        subject: String,
        body: Option<Vec<u8>>,
        timeout_ms: u32,
        max_results: u32,
    ) -> Result<Vec<bindings::exports::wasmcloud::messaging::consumer::BrokerMessage>, String> {
        let body = ::rmp_serde::to_vec_named(&RequestMultiBody {
            subject,
            body,
            timeout_ms,
            max_results,
        })
        .expect("Error when serializing data to send to host");

        let ret_data = bindings::wasmcloud::bus::host::call_sync(
            None,
            "wasmcloud:messaging2/Message.RequestMulti",
            &body,
        )
        .expect("Error when calling host");

        rmp_serde::from_read(std::io::Cursor::new(ret_data))
            .expect("Unable to deserialize body from host")
    }

    fn publish(
        msg: bindings::exports::wasmcloud::messaging::consumer::BrokerMessage,
    ) -> Result<(), String> {
        let body = ::rmp_serde::to_vec_named(&PublishBody { msg })
            .expect("Error when serializing data to send to host");

        // Obviously I could do this differently, but for generated code it'll generate the same and then return nothing for a unit type return
        let _ret_data = bindings::wasmcloud::bus::host::call_sync(
            None,
            "wasmcloud:messaging2/Message.Publish",
            &body,
        )
        .expect("Error when calling host");

        Ok(())
    }
}
