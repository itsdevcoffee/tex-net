use std::error::Error;

use env_logger::Env;
use texnet::{make_client_endpoint, run_client, TexServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize environment variables
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().filter_or("RUST_LOG", "info"));

    let server = TexServer {
        ..Default::default()
    };

    let addr1 = "127.0.0.1:5000".parse().unwrap();
    let addr2 = "127.0.0.1:5051".parse().unwrap();
    let addr3 = "127.0.0.1:5052".parse().unwrap();

    let server1_cert = server.run_server(addr1)?;
    let server2_cert = server.run_server(addr2)?;
    let server3_cert = server.run_server(addr3)?;

    let client = make_client_endpoint(
        "127.0.0.1:0".parse().unwrap(),
        &[&server1_cert, &server2_cert, &server3_cert],
    )?;

    // connect to multiple endpoints using the same socket/endpoint
    tokio::join!(
        run_client(&client, addr1),
        run_client(&client, addr2),
        run_client(&client, addr3),
    );

    // Make sure the server has a chance to clean up
    client.wait_idle().await;

    Ok(())
}
