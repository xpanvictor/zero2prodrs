use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // getting configurations 
    let configuration = get_configuration().expect("Failed to read configurations");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Failed to bind random port");
    
    let addr = listener.local_addr().unwrap();
    eprintln!("Server running at {}", addr);
    run(listener).await?.await
}
