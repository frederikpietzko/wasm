mod prelude;

use bytecodec::DecodeExt;
use env_logger::Builder;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use prelude::*;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_http(req: Request<String>) -> Result<Response<String>> {
    info!(
        "Handling HTTP request: {} {}",
        req.method(),
        req.request_target()
    );
    debug!("Received HTTP request:\n{}", req);

    Ok(Response::new(
        HttpVersion::V1_1,
        StatusCode::new(200)?,
        ReasonPhrase::new("OK")?,
        "Hello World!".to_string(),
    ))
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
    let response = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req),
        Err(e) => Err(anyhow!("Failed to decode request: {}", e)),
    };
    let websocket_upgrade = response.unwrap_or_else(|e| {
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
    Ok(())
}


fn main() -> std::io::Result<()> {
    Builder::new().filter_level(log::LevelFilter::Info).init();
    let port = std::env::var("PORT".to_string()).unwrap_or("8080".to_string());
    let address = format!("0.0.0.0:{}", port);
    info!("Starting server on {}", address);

    let listener = TcpListener::bind(address)?;
    loop {
        let _ = handle_client(listener.accept()?.0);
    }
}
