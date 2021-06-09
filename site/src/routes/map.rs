use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, get, Responder, web};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "map.stpl")]
struct Template {}

#[get("map")]
async fn get() -> impl Responder {
	let html = Template {}
		.render_once()
		.map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
		.unwrap();

	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

pub fn config(cfg: &mut web::ServiceConfig){
	cfg.service(get);
}

pub fn test() -> String {
	let ctx = Template {};
	ctx.render_once().unwrap()
}