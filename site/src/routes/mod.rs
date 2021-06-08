pub mod index
pub mod map

pub fn config(cfg: &mut web::ServiceConfig){
	cfg.service(get);
	index::config(cfg);
	map::config(cfg);
}