use actix_web::{web, get, HttpResponse, Responder};
use super::super::services::UsersServiceFactory;

#[get("")]
async fn find_all() -> impl Responder {
    let service = UsersServiceFactory::create_for_pg_users_repository();
    HttpResponse::Ok().json(service.find_all())
}

// #[post("")]
// async fn create(dto: web::Form<EmailRegistrationDTO>) -> impl Responder {
//     let service = UsersServiceFactory::create_for_pg_users_repository();
//     HttpResponse::Ok().json(service.create())
// }

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/users")
                .service(find_all)
        );
}