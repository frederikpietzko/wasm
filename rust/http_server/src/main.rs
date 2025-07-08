mod prelude;
mod websocket_upgrade;

use bytecodec::DecodeExt;
use env_logger::Builder;
use httpcodec::{HttpVersion, Method, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use prelude::*;
use std::io::{Read, Write};
use wasmedge_wasi_socket::{TcpListener, TcpStream};
use websocket_upgrade::UpgradeResponseHandler;

fn handle_http(req: Request<String>) -> Result<Response<String>> {
    info!(
        "Handling HTTP request: {} {}",
        req.method(),
        req.request_target()
    );
    debug!("Received HTTP request:\n{}", req);
    if (is_upgrade_request(&req)?) {
        let result = UpgradeResponseHandler::new(req).handle_upgrade();
        return result;
    }
    error!(
        "Request to {} {} was not an upgrade request!",
        req.method(),
        req.request_target()
    );
    Err(anyhow!("Not an upgrade request"))
}

fn is_upgrade_request(req: &Request<String>) -> Result<bool> {
    if (req.method() != Method::new("GET")?) {
        return Ok(false);
    }

    if (req.http_version() != HttpVersion::V1_1) {
        return Ok(false);
    }

    if let Some(header) = req.header().get_field("Upgrade")
        && header != "websocket"
    {
        return Ok(false);
    }

    Ok(true)
}

fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        if n == 0 {
            break; // EOF
        }
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break; // No more data
        }
    }
    Ok(data)
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let data = read_stream(&mut stream)?;

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();
    let websocket_upgrade = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req),
        Err(e) => Err(anyhow!("Failed to decode request: {}", e)),
    };
    let websocket_upgrade = websocket_upgrade.unwrap_or_else(|e| {
        let err = format!("{:?}", e);
        Response::new(
            HttpVersion::V1_0,
            StatusCode::new(500).unwrap(),
            ReasonPhrase::new(err.as_str()).unwrap(),
            err.clone(),
        )
    });

    let write_buf = websocket_upgrade.to_string();
    stream.write_all(write_buf.as_bytes())?;
    stream.flush()?;
    websocket_message_loop(stream)?;
    Ok(())
}

fn websocket_message_loop(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0u8; 4096];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                info!("Client disconnected");
                return Ok(());
            }
            Ok(bytes_read) => {
                info!(
                    "Received {} bytes: {:02x?}",
                    bytes_read,
                    &buffer[..bytes_read]
                );
                let first_byte = &buffer[..1];
                info!("Received first byte: {:0b}", first_byte[0]);
            }
            Err(e) => {
                error!("Error reading from stream: {}", e);
                return Err(anyhow!("Error reading from stream: {}", e));
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    Builder::new().filter_level(log::LevelFilter::Info).init();
    let port = std::env::var("PORT".to_string()).unwrap_or("8080".to_string());
    let address = format!("0.0.0.0:{}", port);
    info!("Starting server on {}", address);

    let listener = TcpListener::bind(address, false)?;
    loop {
        let _ = handle_client(listener.accept(false)?.0);
    }
}
