::wit_bindgen::generate!("wasifill");

use std::io::{Read, Write};

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
        let body = ::rmp_serde::to_vec_named(&RequestBody {
            subject,
            body,
            timeout_ms,
        })
        .map_err(|e| e.to_string())?;

        // TODO: I don't know what to do with res here
        let (res, input, output) = wasmcloud::bus::host::call("Message.Request")?;
        let mut output_stream = ::wasmcloud_actor::OutputStreamWriter::from(output);
        output_stream.write_all(&body).map_err(|e| e.to_string())?;

        let input_stream = ::wasmcloud_actor::InputStreamReader::from(input);

        let mut de = ::rmp_serde::Deserializer::new(input_stream);
        match BrokerMessage::deserialize(&mut de) {
            Ok(msg) => {
                let mut reader = de.into_inner();
                if reader.read(&mut [0][..]).unwrap_or_default() > 0 {
                    return Err("unexpected bytes in stream".to_string());
                }
                wait_result(res)?;
                Ok(msg)
            }
            Err(e) => {
                if let Err(err) = wait_result(res) {
                    Err(err)
                } else {
                    Err(e.to_string())
                }
            }
        }
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

        // TODO: I don't know what to do with res here
        let (res, input, output) = wasmcloud::bus::host::call("Message.RequestMulti")?;
        let mut output_stream = ::wasmcloud_actor::OutputStreamWriter::from(output);

        output_stream.write_all(&body).map_err(|e| e.to_string())?;
        let input_stream = ::wasmcloud_actor::InputStreamReader::from(input);

        // Ugly hack to get around remote derive of type
        #[derive(::serde::Deserialize)]
        struct Wrapper(#[serde(with = "BrokerMessage")] wasmcloud::messaging::types::BrokerMessage);

        let mut de = ::rmp_serde::Deserializer::new(input_stream);
        match <Vec<_> as ::serde::Deserialize>::deserialize(&mut de)
            .map(|v| v.into_iter().map(|Wrapper(m)| m).collect())
        {
            Ok(msgs) => {
                let mut reader = de.into_inner();
                if reader.read(&mut [0][..]).unwrap_or_default() > 0 {
                    return Err("unexpected bytes in stream".to_string());
                }
                wait_result(res)?;
                Ok(msgs)
            }
            Err(e) => {
                if let Err(err) = wait_result(res) {
                    Err(err)
                } else {
                    Err(e.to_string())
                }
            }
        }
    }

    fn publish(
        msg: exports::wasmcloud::messaging::consumer::BrokerMessage,
    ) -> Result<(), ::wit_bindgen::rt::string::String> {
        let body = ::rmp_serde::to_vec_named(&PublishBody { msg }).map_err(|e| e.to_string())?;

        // TODO: I don't know what to do with res here
        let (res, _input, output) = wasmcloud::bus::host::call("Message.Publish")?;
        let mut output_stream = ::wasmcloud_actor::OutputStreamWriter::from(output);

        output_stream.write_all(&body).map_err(|e| e.to_string())?;
        // NOTE(thomastaylor312): I don't think we need to read anything back from the stream, but
        // maybe we should be reading something back for completeness?
        // let input_stream = ::wasmcloud_actor::InputStreamReader::from(input);

        wait_result(res)
    }
}

export_wasifill!(WasifillImpl);

// This is the most hacky form of a poll loop until things start working for real and we use async
// instead
fn wait_result(res: wasmcloud::bus::host::FutureResult) -> Result<(), String> {
    loop {
        if let Some(r) = wasmcloud::bus::host::future_result_get(res) {
            return r;
        } else {
            ::std::thread::sleep(::std::time::Duration::from_millis(5))
        }
    }
}
