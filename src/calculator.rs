use std::sync;

use tokio::sync::RwLock;

use crate::proto::{self, calculator_server::Calculator};

type State = sync::Arc<RwLock<u64>>;

#[derive(Debug, Default)]
pub struct CalculatorService {
    pub state: State,
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
