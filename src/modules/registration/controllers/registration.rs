use actix_web::{web, post, HttpResponse, Responder};
use super::super::{services::RegistrationServiceFactory, dto::EmailRegistrationDTO};

#[post("/email")]
async fn register_by_email(dto: web::Form<EmailRegistrationDTO>) -> impl Responder {
    let service = RegistrationServiceFactory::create_for_email_strategy();
    let result = service.register(&dto);

    match result {
        Ok(_) => HttpResponse::Ok().body("registered by email"),
        Err(error) => {
            HttpResponse::BadRequest().json(error)
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/registration").service(register_by_email));
}