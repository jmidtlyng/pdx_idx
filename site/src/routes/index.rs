use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Template {
	site_name: String, 
	state: String,
	region: String
}

#[get("")]
async fn get() -> impl Responder {
	let site_name = "PDX IDX";
	let state = "Oregon";
	let region = "Portland";
	
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
	let ctx = Template { 
		site_name: "PDX IDX",
		state: "Oregon",
		region: "Portland"
	};
	ctx.render_once().unwrap()
}