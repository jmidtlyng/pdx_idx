use actix_web::web;

pub mod index;
pub mod map;

pub fn config(cfg: &mut web::ServiceConfig){
	index::config(cfg);
	map::config(cfg);
}