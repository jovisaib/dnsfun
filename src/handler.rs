use crate::Options;
use trust_dns_server::server::{Request, RequestHandler, ResponseHandler, ResponseInfo};


/// DNS Request handler
#[derive(Clone, Debug)]
pub struct Handler {}

impl Handler {
    pub fn from_options(_options: &Options) -> Self {
        Handler {}
    } 
}

#[async_trait::async_trait]
impl RequestHandler for Handler {
    async fn handle_request<R: ResponseHandler>(
        &self,
        _request: &Request,
        _response: R,
    ) -> ResponseInfo{
        todo!()
    }
}