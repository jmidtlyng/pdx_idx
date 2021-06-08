use actix_web::web;
use crate::routes;

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service( web::scope("/").configure(routes::config) );
}