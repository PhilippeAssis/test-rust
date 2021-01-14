#[macro_use]
extern crate serde_derive;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Path;
extern crate redirect_rules;
use redirect_rules::cara_legal;
use std::ops::Deref;

#[derive(Deserialize, Debug)]
struct Info {
    username: String,
}

impl Info {
    fn get_name(&self) -> String {
        self.username.to_owned()
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo/{username}")]
async fn echo(info: Path<Info>) -> impl Responder {
    let res = cara_legal(&info.username);
    HttpResponse::Ok().body(res)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
