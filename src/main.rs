use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("There are Ping and his brother Pong from China :)")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    use ru_tsk_dv::modules::registration;
    use ru_tsk_dv::modules::users;
    
    let registration_service_factory = web::Data::new(registration::services::RegistrationServiceFactory);
    let users_service = web::Data::new(users::services::UsersServiceFactory::create_for_pg());

    HttpServer::new(move || {
        App::new()
            .app_data(registration_service_factory.clone())
            .app_data(users_service.clone())
            .service(ping)
            .configure(registration::controllers::init)
            .configure(users::controllers::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
