extern crate feedapi;
extern crate grpcio;

use feedapi::feedapi::{Contractor, CreateContractorRequest};
use feedapi::feedapi_grpc::FeedApiClient;
use grpcio::{ChannelBuilder, EnvBuilder};
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn create_contractor() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50505");
    let client = FeedApiClient::new(ch);

    let mut app = Contractor::new();
    app.set_ID("app".to_string());
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("p".to_string(), "v".to_string());
    app.set_Params(params);

    let mut req = CreateContractorRequest::new();
    req.set_Contractor(app);

    let response = client.create_contractor(&req).unwrap();

    assert!(!response.has_Error());
}
