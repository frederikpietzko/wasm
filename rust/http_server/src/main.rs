use std::io::{Read, Write};

use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use wasmedge_wasi_socket::{TcpListener, TcpStream};

fn handle_http(req: Request<String>) -> bytecodec::Result<Response<String>> {
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("OK")?,
        format!("echo, {}!", req.body()),
    ))
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }
    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();
    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req),
        Err(e) => Err(e),
    };

    let r = req.unwrap_or_else(|e| {
        let err = format!("{:?}", e);
        Response::new(
            HttpVersion::V1_0,
            StatusCode::new(500).unwrap(),
            ReasonPhrase::new(err.as_str()).unwrap(),
            err.clone(),
        )
    });

    let write_buf = r.to_string();
    stream.write(write_buf.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Both)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT".to_string()).unwrap_or("8080".to_string());
    println!("starting server at {}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false)?;
    loop {
        let _ = handle_client(listener.accept(false)?.0);
    }
}
