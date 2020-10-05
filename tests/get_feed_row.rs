extern crate feedapi;
extern crate grpcio;

use feedapi::feedapi::GetFeedRowRequest;
use feedapi::feedapi_grpc::FeedApiClient;
use grpcio::{ChannelBuilder, EnvBuilder};
use protobuf::RepeatedField;
use std::sync::Arc;

#[test]
fn get_feed_row() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50505");
    let client = FeedApiClient::new(ch);

    let mut req = GetFeedRowRequest::new();
    req.set_AccountID(String::from("f72f322b-633f-4804-a6e2-59323e892c48"));
    let rows = vec![
        "96c68f45-f807-494a-ac1d-77738d018f47".to_string(),
        "06fd05e2-1170-4a3f-abec-df4fed2ff2a4".to_string(),
    ];

    req.set_FeedRowIDs(RepeatedField::from_vec(rows));

    match client.get_feed_row(&req) {
        Ok(r) => println!("{:?}", r),
        Err(e) => eprintln!("{:?}", e),
    };
}
