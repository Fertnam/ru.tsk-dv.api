use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("There are Ping and his brother Pong from China :)")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    use ru_tsk_dv::modules::registration;

    HttpServer::new(|| {
        App::new()
            .service(ping)
            .configure(registration::controllers::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
