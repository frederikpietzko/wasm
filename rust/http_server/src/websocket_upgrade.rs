use crate::prelude::*;
use base64::engine::general_purpose;
use base64::Engine;
use httpcodec::{HeaderField, HttpVersion, ReasonPhrase, Request, Response, StatusCode};
use sha1::{Digest, Sha1};

const WEBSOCKET_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub struct UpgradeResponseHandler {
    request: Request<String>,
}

impl UpgradeResponseHandler {
    pub fn new(request: Request<String>) -> Self {
        UpgradeResponseHandler { request }
    }

    fn compute_sec_websocket_accept(&self) -> Result<String> {
        let header = self.request.header();
        let sec_websocket_key = header
            .get_field("Sec-WebSocket-Key")
            .ok_or_else(|| anyhow!("Missing Sec-WebSocket-Key header"))?;

        let mut hasher = Sha1::new();
        hasher.update(sec_websocket_key.trim().as_bytes());
        hasher.update(WEBSOCKET_GUID.as_bytes());
        let result = hasher.finalize();
        Ok(general_purpose::STANDARD.encode(result))
    }

    pub fn handle_upgrade(&mut self) -> Result<Response<String>> {
        let mut response = Response::new(
            HttpVersion::V1_1,
            StatusCode::new(101)?,
            ReasonPhrase::new("Switching Protocols")?,
            "".to_string(),
        );
        let mut headers = response.header_mut();
        headers.add_field(HeaderField::new("Upgrade", "websocket")?);
        headers.add_field(HeaderField::new("Connection", "upgrade")?);

        let sec_websocket_accept = self.compute_sec_websocket_accept()?;

        headers.add_field(HeaderField::new(
            "Sec-WebSocket-Accept",
            &*sec_websocket_accept,
        )?);

        Ok(response)
    }
}
