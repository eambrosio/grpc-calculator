use std::sync;

use proto::calculator_server::CalculatorServer;
use tokio::sync::RwLock;
use tonic::transport::Server;

use crate::{
    admin::{check_auth, AdminService},
    calculator::CalculatorService,
    proto::{admin_server::AdminServer},
};

mod admin;
mod calculator;
mod proto;

type State = sync::Arc<RwLock<u64>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;

    let state = State::default();
    let calc = CalculatorService {
        state: state.clone(),
    };
    let admin = AdminService {
        state: state.clone(),
    };

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    Server::builder()
        .add_service(reflection_service)
        .add_service(CalculatorServer::new(calc))
        .add_service(AdminServer::with_interceptor(admin, check_auth))
        .serve(addr)
        .await?;

    Ok(())
}
