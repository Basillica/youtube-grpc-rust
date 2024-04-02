use auth_service::auth_service_client::AuthServiceClient;
use auth_service::SignInReq;

pub mod auth_service {
    tonic::include_proto!("auth");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AuthServiceClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(SignInReq {
        username: "Tonie".into(),
        password: "password".into(),
    });

    let response = client.sign_in(request).await?;
    println!("response: {:?}", response);
    Ok(())
}
