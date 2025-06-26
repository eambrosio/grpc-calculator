use std::sync;

use axum::Router;
use tokio::sync::RwLock;

use crate::calculator::CalculatorService;

mod admin;
mod calculator;
mod proto;

type State = sync::Arc<RwLock<u64>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = State::default();
    let calc = CalculatorService {
        state: state.clone(),
    };

    let http_router = proto::calculator_handler(calc);
    let _calculator_http_app = Router::new().nest("/api", http_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, _calculator_http_app).await.unwrap();

    Ok(())
}
