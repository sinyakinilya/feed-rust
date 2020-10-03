use std::io::Read;
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use std::{io, thread};

use feedapi::feedapi::*;
use feedapi::feedapi_grpc::{self, FeedApi};
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::prelude::*;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use protobuf::{well_known_types::Timestamp, RepeatedField};

mod config;
mod mongo;

#[derive(Clone)]
struct FeedApiService {
    feed_collection: Arc<mongo::FeedCollection>,
}

impl FeedApiService {
    fn new(col: mongo::FeedCollection) -> Self {
        FeedApiService {
            feed_collection: Arc::new(col),
        }
    }
}

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
        let rows_id = req.get_FeedRowIDs();
        let rows = self.feed_collection.find_feed(rows_id.to_vec());
        let mut get_feed_row_response = GetFeedRowResponse::new();
        let mut rsp_row = vec![];
        for row in rows.iter() {
            let mut t = Timestamp::new();
            let duration = row.created_at.duration_since(UNIX_EPOCH).unwrap();
            t.set_seconds(duration.as_secs() as i64);
            t.set_nanos(duration.subsec_nanos() as i32);

            let mut r = GetFeedRowResponse_Row::new();
            r.set_ID(row.id.to_string());
            r.set_AccountID(row.account_id.to_string());
            r.set_PartnerAccountID(row.partner_account_id.to_string());
            r.set_CreatedAt(t);
            rsp_row.push(r);
        }
        get_feed_row_response.set_Rows(RepeatedField::from_vec(rsp_row));
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

    fn create_contractor(
        &mut self,
        ctx: RpcContext,
        req: CreateContractorRequest,
        sink: UnarySink<CreateContractorResponse>,
    ) {
        println!("Received CreateContractorRequest {{ {:?} }}", req);
        let create_contractor_response = CreateContractorResponse::new();

        let f = sink
            .success(create_contractor_response.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| {
                println!(
                    "Responded with CreateContractorRequest {{ {:?} }}",
                    create_contractor_response
                )
            });

        ctx.spawn(f)
    }

    fn update_contractor(
        &mut self,
        ctx: RpcContext,
        req: UpdateContractorRequest,
        sink: UnarySink<UpdateContractorResponse>,
    ) {
        println!("Received UpdateContractorResponse {{ {:?} }}", req);
        let update_contractor_response = UpdateContractorResponse::new();

        let f = sink
            .success(update_contractor_response.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| {
                println!(
                    "Responded with UpdateContractorResponse {{ {:?} }}",
                    update_contractor_response
                )
            });

        ctx.spawn(f)
    }

    fn get_contractors(
        &mut self,
        ctx: RpcContext,
        req: GetContractorsRequest,
        sink: UnarySink<GetContractorsResponse>,
    ) {
        println!("Received GetContractorsResponse {{ {:?} }}", req);
        let get_contractor_response = GetContractorsResponse::new();

        let f = sink
            .success(get_contractor_response.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| {
                println!(
                    "Responded with GetContractorsResponse {{ {:?} }}",
                    get_contractor_response
                )
            });

        ctx.spawn(f)
    }

    fn have_seen(
        &mut self,
        ctx: RpcContext,
        req: HaveSeenRequest,
        sink: UnarySink<HaveSeenResponse>,
    ) {
        println!("Received HaveSeenResponse {{ {:?} }}", req);
        let have_seen_response = HaveSeenResponse::new();

        let f = sink
            .success(have_seen_response.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| {
                println!(
                    "Responded with HaveSeenResponse {{ {:?} }}",
                    have_seen_response
                )
            });

        ctx.spawn(f)
    }

    fn seen(&mut self, ctx: RpcContext, req: SeenRequest, sink: UnarySink<SeenResponse>) {
        println!("Received SeenResponse {{ {:?} }}", req);
        let seen_response = SeenResponse::new();

        let f = sink
            .success(seen_response.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| println!("Responded with SeenResponse {{ {:?} }}", seen_response));

        ctx.spawn(f)
    }
}
fn main() {
    let env = Arc::new(Environment::new(1));
    let consul_cfg = config::resolve_cfg().unwrap();
    let f_collection = mongo::FeedCollection::new(&consul_cfg.mongo.url);
    let feed_service = FeedApiService::new(f_collection);
    let service = feedapi_grpc::create_feed_api(feed_service);
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
