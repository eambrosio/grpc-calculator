use std::sync;

use proto::calculator_server::{Calculator, CalculatorServer};
use tokio::sync::RwLock;
use tonic::transport::Server;

use crate::proto::admin_server::{Admin, AdminServer};

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

type State = sync::Arc<RwLock<u64>>;
#[derive(Debug, Default)]
struct CalculatorService {
    state: State,
}

#[derive(Debug, Default)]
struct AdminService {
    state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self,
        _request: tonic::Request<proto::GetRequestCountRequest>,
    ) -> Result<tonic::Response<proto::GetRequestCountResponse>, tonic::Status> {
        let count = self.state.read().await;
        let response = tonic::Response::new(proto::GetRequestCountResponse { count: *count });

        Ok(response)
    }
}

impl CalculatorService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
        println!("Request count: {}", *count);
    }
}
#[tonic::async_trait]
impl Calculator for CalculatorService {
    #[doc = " Adds two integers and returns the result."]
    async fn add(
        &self,
        request: tonic::Request<proto::CalculatorRequest>,
    ) -> Result<tonic::Response<proto::CalculatorResponse>, tonic::Status> {
        let input = request.get_ref();

        let response = proto::CalculatorResponse {
            result: input.a + input.b,
        };

        self.increment_counter().await;
        Ok(tonic::Response::new(response))
    }

    async fn divide(
        &self,
        request: tonic::Request<proto::CalculatorRequest>,
    ) -> Result<tonic::Response<proto::CalculatorResponse>, tonic::Status> {
        let input = request.get_ref();

        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("Cannot divide by zero"));
        }

        let response = proto::CalculatorResponse {
            result: input.a / input.b,
        };

        self.increment_counter().await;

        Ok(tonic::Response::new(response))
    }
}

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
        .add_service(AdminServer::new(admin))
        .serve(addr)
        .await?;

    Ok(())
}
