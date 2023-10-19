use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    Index {
        name: "Actix".to_string(),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
