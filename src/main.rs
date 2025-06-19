use proto::calculator_server::{Calculator, CalculatorServer};
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("calculator");
}

#[derive(Debug, Default)]
struct CalculatorService {}

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

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;

    let calc = CalculatorService::default();

    Server::builder()
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    Ok(())
}
