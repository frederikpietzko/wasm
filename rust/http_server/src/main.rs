use bytecodec::DecodeExt;
use httpcodec::{
    HeaderField, HttpVersion, Method, ReasonPhrase, Request, RequestDecoder, Response, StatusCode,
};
use sha1::{Digest, Sha1};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{TcpListener, TcpStream};

fn handle_http(req: Request<String>) -> bytecodec::Result<Response<String>> {
    if (is_upgrade_request(&req)?) {
        let mut response = Response::new(
            HttpVersion::V1_1,
            StatusCode::new(101)?,
            ReasonPhrase::new("Switching Protocols")?,
            "Switching to WebSocket protocol".to_string(),
        );
        let mut headers = response.header_mut();
        headers.add_field(HeaderField::new("Upgrade", "websocket")?);
        headers.add_field(HeaderField::new("Connection", "Upgrade")?);
        let guid = uuid::Uuid::new_v4();
        todo!("Add WebSocket Sec-WebSocket-Accept header");
        // Websocket server should calculate Sec-WebSocket-Accept by
        // taking Sec-WebSocket-Key from the request and
        // applying the SHA-1 hash with the WebSocket GUID.
        // base64 encode the result and set it in the response.

        return Ok(response);
    }
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("OK")?,
        format!("echo, {}!", req.body()),
    ))
}

fn is_upgrade_request(req: &Request<String>) -> bytecodec::Result<bool> {
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
    let mut hasher = Sha1::new();
    hasher.update(b"hello world");
    let hash = hasher.finalize();
    println!("SHA1 hash of 'hello world': {:x}", hash);
    let port = std::env::var("PORT".to_string()).unwrap_or("8080".to_string());
    println!("starting server at {}", port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false)?;
    loop {
        let _ = handle_client(listener.accept(false)?.0);
    }
}
