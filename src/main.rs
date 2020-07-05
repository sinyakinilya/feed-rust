use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use feedapi::feedapi::*;
use feedapi::feedapi_grpc::{self, FeedApi};
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

#[derive(Clone)]
struct FeedApiService;

impl FeedApi for FeedApiService {
    fn create_feed_row(
        &mut self,
        ctx: RpcContext,
        req: CreateFeedRowRequest,
        sink: UnarySink<CreateFeedRowResponse>,
    ) {
        println!("Received CreateFeedRowRequest {{ {:?} }}", req);
        let create_feed_response = CreateFeedRowResponse::new();

        let f = sink
            .success(create_feed_response.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| {
                println!(
                    "Responded with CreateFeedRowResponse {{ {:?} }}",
                    create_feed_response
                )
            });
        ctx.spawn(f)
    }
    fn add_state_to_feed_row(
        &mut self,
        ctx: RpcContext,
        req: AddStateToFeedRowRequest,
        sink: UnarySink<AddStateToFeedRowResponse>,
    ) {
        println!("Received AddStateToFeedRowRequest {{ {:?} }}", req);
        let add_state_resp = AddStateToFeedRowResponse::new();

        let f = sink
            .success(add_state_resp.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| {
                println!(
                    "Responded with AddStateToFeedRowResponse {{ {:?} }}",
                    add_state_resp
                )
            });
        ctx.spawn(f)
    }

    fn update_state(
        &mut self,
        ctx: RpcContext,
        req: UpdateStateRequest,
        sink: UnarySink<UpdateStateResponse>,
    ) {
        println!("Received UpdateStateRequest {{ {:?} }}", req);
        let update_state_response = UpdateStateResponse::new();

        let f = sink
            .success(update_state_response.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| {
                println!(
                    "Responded with UpdateStateResponse {{ {:?} }}",
                    update_state_response
                )
            });
        ctx.spawn(f)
    }
    fn get_feed(&mut self, ctx: RpcContext, req: GetFeedRequest, sink: UnarySink<GetFeedResponse>) {
        println!("Received GetFeedRequest {{ {:?} }}", req);
        let get_feed_response = GetFeedResponse::new();

        let f = sink
            .success(get_feed_response.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| {
                println!(
                    "Responded with GetFeedResponse {{ {:?} }}",
                    get_feed_response
                )
            });
        ctx.spawn(f)
    }

    fn get_feed_row(
        &mut self,
        ctx: RpcContext,
        req: GetFeedRowRequest,
        sink: UnarySink<GetFeedRowResponse>,
    ) {
        println!("Received GetFeedRowResponse {{ {:?} }}", req);
        let get_feed_row_response = GetFeedRowResponse::new();

        let f = sink
            .success(get_feed_row_response.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| {
                println!(
                    "Responded with GetFeedRowResponse {{ {:?} }}",
                    get_feed_row_response
                )
            });
        ctx.spawn(f)
    }
}
fn main() {
    let env = Arc::new(Environment::new(1));
    let service = feedapi_grpc::create_feed_api(FeedApiService);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50505)
        .build()
        .unwrap();
    server.start();
    for (ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}
