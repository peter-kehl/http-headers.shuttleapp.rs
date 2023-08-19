use actix_web::{get, web::ServiceConfig};
use actix_web::{Error, HttpRequest, HttpResponse};
use mime::TEXT_PLAIN_UTF_8;
use shuttle_actix_web::ShuttleActixWeb;

const NON_ASCII_IN_HEADER: &str = "Non-ASCII character(s) in the header.";

#[get("/")]
async fn hello_world(req: HttpRequest) -> Result<HttpResponse, Error> {
    let mut lines = Vec::with_capacity(req.headers().len() + 1);
    for (key, value) in req.headers() {
        let value = match value.to_str() {
            Ok(s) => s,
            Err(_) => NON_ASCII_IN_HEADER,
        };
        lines.push(format!("{}: {}", key.to_string(), value));
    }
    let peer_addr = if let Some(peer_addr) = req.peer_addr() {
        peer_addr.ip().to_string()
    } else {
        NON_ASCII_IN_HEADER.to_owned()
    };
    lines.push(format!("Client IP address: {:?}", peer_addr));

    Ok(HttpResponse::Ok()
        .content_type(TEXT_PLAIN_UTF_8)
        .body(lines.join("\n")))
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}
