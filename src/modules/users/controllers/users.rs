use actix_web::{web, get, post, HttpResponse, Responder};
use super::super::services::UsersService;
use super::super::dto::UserCreationDTO;

#[get("")]
async fn find_all(users_service: web::Data<UsersService>) -> impl Responder {
    HttpResponse::Ok().json(users_service.find_all())
}

#[post("")]
async fn create(dto: web::Form<UserCreationDTO>, users_service: web::Data<UsersService>) -> impl Responder {
    HttpResponse::Ok().json(users_service.create(&dto))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/users")
                .service(find_all)
                .service(create)
        );
}