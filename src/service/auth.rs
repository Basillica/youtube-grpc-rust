use auth_service::auth_service_server::{AuthService, AuthServiceServer};
use auth_service::{SignInReq, SignInResp, SignOutReq, SignOutResp, User};
use tonic::{Request, Response, Status};

pub mod auth_service {
    tonic::include_proto!("auth");
}

#[derive(Debug, Default)]
pub struct AuthenticationService {}

#[tonic::async_trait]
impl AuthService for AuthenticationService {
    async fn sign_in(&self, request: Request<SignInReq>) -> Result<Response<SignInResp>, Status> {
        let user: SignInReq = request.into_inner();
        println!("Got a response from the following user: {:?}", user);
        let reply = SignInResp {
            user: Some(auth_service::User {
                first_name: String::from("tonie"),
                last_name: String::from("tonie"),
                email: String::from("tonie"),
            }),
            token: String::from("tonie"),
            refresh_token: String::from("tonie"),
        };
        Ok(Response::new(reply))
    }

    async fn sign_out(
        &self,
        request: Request<SignOutReq>,
    ) -> Result<Response<SignOutResp>, Status> {
        let user: SignOutReq = request.into_inner();
        println!("Got a response from the following user: {:?}", user);
        let reply = SignOutResp {};
        Ok(Response::new(reply))
    }

    async fn sign_up(&self, request: Request<User>) -> Result<Response<User>, Status> {
        let user: User = request.into_inner();
        println!("Got a response from the following user: {:?}", user);
        let reply = User {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        };
        Ok(Response::new(reply))
    }

    async fn update(&self, request: Request<User>) -> Result<Response<User>, Status> {
        let user: User = request.into_inner();
        println!("Got a response from the following user: {:?}", user);
        let reply = User {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        };
        Ok(Response::new(reply))
    }
}

pub fn authentication_service() -> AuthServiceServer<AuthenticationService> {
    let auth = AuthenticationService::default();
    AuthServiceServer::new(auth)
}
