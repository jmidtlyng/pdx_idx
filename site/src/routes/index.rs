use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, get, Responder, web};
use sailfish::TemplateOnce;
use crate::config::get_config;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Template {
	site_name: String, 
	state: String,
	region: String
}

#[get("")]
async fn get() -> impl Responder {
	let config = get_config().expect("Failed to read configuration.");

	let site_name = config.site_name;
	let state = config.state;
	let region = config.region;
	
	let html = Template { site_name, state, region }
		.render_once()
		.map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
		.unwrap();

	HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

pub fn config(cfg: &mut web::ServiceConfig){
	cfg.service(get);
}

pub fn test() -> String {
	let config = get_config().expect("Failed to read configuration.");

	let ctx = Template { 
		site_name: config.site_name,
		state: config.state,
		region: config.region
	};
	ctx.render_once().unwrap()
}