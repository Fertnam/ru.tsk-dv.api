use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use ru_tsk_dv::users as usersModule;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/users")]
async fn users() -> impl Responder {
    HttpResponse::Ok().json(usersModule::find_all::find_all())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
