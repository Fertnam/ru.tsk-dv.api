use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/users/register")]
async fn users_register() -> impl Responder {
    use ru_tsk_dv::modules::registration::{ services::RegistrationServiceFactory, dto::EmailRegistrationDTO };

    let service = RegistrationServiceFactory::create_for_email_strategy();

    let dto = EmailRegistrationDTO {
        email: String::from("fertnam@tsk.dv.ru"), 
        password: String::from(""), 
        name: String::from("")
    };

    service.register(&dto);

    HttpResponse::Ok().body("usersRegister")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(users_register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
