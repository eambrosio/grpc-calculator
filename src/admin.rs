use std::sync;

use tokio::sync::RwLock;
use tonic::metadata::MetadataValue;

use crate::proto::{self, admin_server::Admin};
type State = sync::Arc<RwLock<u64>>;

#[derive(Debug, Default)]
pub struct AdminService {
    pub state: State,
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

pub fn check_auth(req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    let token: MetadataValue<_> = "Bearer token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if t == token => Ok(req),
        _ => Err(tonic::Status::unauthenticated("Invalid auth token")),
    }
}
