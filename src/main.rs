use actix_web::{get, web, App, HttpServer, Responder};
use base64::{Engine, engine::general_purpose};
use std::prelude;

#[get("/e/{string_data}")]
async fn index(string_data: web::Path<String>) -> impl Responder {
    general_purpose::STANDARD.encode(string_data.as_bytes())
}

#[get("/d/{string_data}")]
async fn hello(string_data: web::Path<String>) -> impl Responder {
    let bytes = general_purpose::STANDARD.decode(string_data.as_str()).unwrap();
    // let bytes = base64::decode("YmFzZTY0IGRlY29kZQ==").unwrap();
    let str = String::from_utf8(bytes).unwrap();
    println!("{}", &str);
    str
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("0.0.0.0", 8088))?
        .run()
        .await
}