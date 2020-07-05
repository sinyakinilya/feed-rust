use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, UnarySink};
use feedapi::feedapi::*;
use feedapi::feedapi_grpc::{self, FeedApi};

#[derive(Clone)]
struct FeedApiService;

impl FeedApi for FeedApiService {
    fn create_feed_row(
        &mut self,
        ctx: RpcContext,
        req: CreateFeedRowRequest,
        sink: UnarySink<CreateFeedRowResponse>,
    ) {
        println!("Received Create Feed {{ {:?} }}", req);
        let mut feed = CreateFeedRowResponse::new();

        let f = sink
            .success(feed.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| println!("Responded with Check {{ {:?} }}", check));
        ctx.spawn(f)
    }
    fn add_state_to_feed_row(
        &mut self,
        ctx: RpcContext,
        req: AddStateToFeedRowRequest,
        sink: UnarySink<AddStateToFeedRowResponse>,
    ) {
        println!("Received Add State To Feed {{ {:?} }}", req);
        let mut feed = CreateFeedRowResponse::new();

        let f = sink
            .success(feed.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| println!("Responded with Check {{ {:?} }}", check));
        ctx.spawn(f)

    }
    fn update_state(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::feedapi::UpdateStateRequest,
        sink: ::grpcio::UnarySink<super::feedapi::UpdateStateResponse>,
    );
    fn get_feed(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::feedapi::GetFeedRequest,
        sink: ::grpcio::UnarySink<super::feedapi::GetFeedResponse>,
    );
    fn get_feed_row(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: super::feedapi::GetFeedRowRequest,
        sink: ::grpcio::UnarySink<super::feedapi::GetFeedRowResponse>,
    );
}
fn main() {

    // let env = Arc::new(Environment::new(1));
    // let service = create_greeter(GreeterService);

    // let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(1024 * 1024);
    // let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    // let mut server = ServerBuilder::new(env)
    //     .register_service(service)
    //     .bind("127.0.0.1", 50_051)
    //     .channel_args(ch_builder.build_args())
    //     .build()
    //     .unwrap();
    // server.start();
    // for (host, port) in server.bind_addrs() {
    //     info!("listening on {}:{}", host, port);
    // }
    // let (tx, rx) = oneshot::channel();
    // thread::spawn(move || {
    //     info!("Press ENTER to exit...");
    //     let _ = io::stdin().read(&mut [0]).unwrap();
    //     tx.send(())
    // });
    // let _ = block_on(rx);
    // let _ = block_on(server.shutdown());
}
