use wstd::http::{Body, Request, Response, Result, StatusCode};

#[wstd::http_server]
async fn main(req: Request<Body>) -> Result<Response<Body>> {
    let body = match req.uri().path_and_query().unwrap().as_str() {
        "/echo" => echo(req).await,
        _ => not_found().await,
    };
    Ok(body)
}

async fn echo(mut req: Request<Body>) -> Response<Body> {
    let body = req.body_mut().str_contents().await.unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(body))
        .unwrap()
}

async fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}
