use std::net::TcpListener;
use site::server::serve;
use site::routes;

pub struct Site {
	pub address: String
}

async fn spawn_app() -> Site {
		let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
		let port = listener.local_addr().unwrap().port();
		let address = format!("http://127.0.0.1:{}", port);
		let server = serve(listener).expect("Failed to bind address");
		let _ = tokio::spawn(server);
		Site { address }
}

#[actix_rt::test]
async fn server_check() {
		let app = spawn_app().await;
		let client = reqwest::Client::new();

		template_rendering_check(app, client).await;
		crud_check(app, client).await;
}

async fn template_rendering_check(app: Site, client: reqwest::Client){
		let routes: &[(&str, String); 3] = &[
				("/", routes::index::test()),
				("map", routes::map::test())
		];
		
		for route in routes {
				let uri = &format!("{}/{}", &app.address, route.0);
				let response = client.get(uri).send().await
						.expect("Failed to execute request.");
				let response_html = response.text().await.unwrap();
				
				assert_eq!(route.1, response_html);
		}
}