use service::auth::authentication_service;
use tonic::transport::Server;

mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    Server::builder()
        .add_service(authentication_service())
        .serve(addr)
        .await?;

    Ok(())
}
