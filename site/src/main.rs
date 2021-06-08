use site::config::get_config;
use std::net::TcpListener;
use crate::server::serve;
    
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    serve(listener)?.await
}