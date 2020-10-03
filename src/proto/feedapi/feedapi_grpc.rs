// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_FEED_API_CREATE_FEED_ROW: ::grpcio::Method<super::feedapi::CreateFeedRowRequest, super::feedapi::CreateFeedRowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/CreateFeedRow",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FEED_API_ADD_STATE_TO_FEED_ROW: ::grpcio::Method<super::feedapi::AddStateToFeedRowRequest, super::feedapi::AddStateToFeedRowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/AddStateToFeedRow",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FEED_API_UPDATE_STATE: ::grpcio::Method<super::feedapi::UpdateStateRequest, super::feedapi::UpdateStateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/UpdateState",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FEED_API_GET_FEED: ::grpcio::Method<super::feedapi::GetFeedRequest, super::feedapi::GetFeedResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/GetFeed",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FEED_API_GET_FEED_ROW: ::grpcio::Method<super::feedapi::GetFeedRowRequest, super::feedapi::GetFeedRowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/GetFeedRow",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FEED_API_CREATE_CONTRACTOR: ::grpcio::Method<super::feedapi::CreateContractorRequest, super::feedapi::CreateContractorResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/CreateContractor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FEED_API_UPDATE_CONTRACTOR: ::grpcio::Method<super::feedapi::UpdateContractorRequest, super::feedapi::UpdateContractorResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/UpdateContractor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FEED_API_GET_CONTRACTORS: ::grpcio::Method<super::feedapi::GetContractorsRequest, super::feedapi::GetContractorsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/GetContractors",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FEED_API_HAVE_SEEN: ::grpcio::Method<super::feedapi::HaveSeenRequest, super::feedapi::HaveSeenResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/HaveSeen",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_FEED_API_SEEN: ::grpcio::Method<super::feedapi::SeenRequest, super::feedapi::SeenResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/feedapi.FeedAPI/Seen",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct FeedApiClient {
    client: ::grpcio::Client,
}

impl FeedApiClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        FeedApiClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_feed_row_opt(&self, req: &super::feedapi::CreateFeedRowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::CreateFeedRowResponse> {
        self.client.unary_call(&METHOD_FEED_API_CREATE_FEED_ROW, req, opt)
    }

    pub fn create_feed_row(&self, req: &super::feedapi::CreateFeedRowRequest) -> ::grpcio::Result<super::feedapi::CreateFeedRowResponse> {
        self.create_feed_row_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_feed_row_async_opt(&self, req: &super::feedapi::CreateFeedRowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::CreateFeedRowResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_CREATE_FEED_ROW, req, opt)
    }

    pub fn create_feed_row_async(&self, req: &super::feedapi::CreateFeedRowRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::CreateFeedRowResponse>> {
        self.create_feed_row_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_state_to_feed_row_opt(&self, req: &super::feedapi::AddStateToFeedRowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::AddStateToFeedRowResponse> {
        self.client.unary_call(&METHOD_FEED_API_ADD_STATE_TO_FEED_ROW, req, opt)
    }

    pub fn add_state_to_feed_row(&self, req: &super::feedapi::AddStateToFeedRowRequest) -> ::grpcio::Result<super::feedapi::AddStateToFeedRowResponse> {
        self.add_state_to_feed_row_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_state_to_feed_row_async_opt(&self, req: &super::feedapi::AddStateToFeedRowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::AddStateToFeedRowResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_ADD_STATE_TO_FEED_ROW, req, opt)
    }

    pub fn add_state_to_feed_row_async(&self, req: &super::feedapi::AddStateToFeedRowRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::AddStateToFeedRowResponse>> {
        self.add_state_to_feed_row_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_state_opt(&self, req: &super::feedapi::UpdateStateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::UpdateStateResponse> {
        self.client.unary_call(&METHOD_FEED_API_UPDATE_STATE, req, opt)
    }

    pub fn update_state(&self, req: &super::feedapi::UpdateStateRequest) -> ::grpcio::Result<super::feedapi::UpdateStateResponse> {
        self.update_state_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_state_async_opt(&self, req: &super::feedapi::UpdateStateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::UpdateStateResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_UPDATE_STATE, req, opt)
    }

    pub fn update_state_async(&self, req: &super::feedapi::UpdateStateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::UpdateStateResponse>> {
        self.update_state_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_feed_opt(&self, req: &super::feedapi::GetFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::GetFeedResponse> {
        self.client.unary_call(&METHOD_FEED_API_GET_FEED, req, opt)
    }

    pub fn get_feed(&self, req: &super::feedapi::GetFeedRequest) -> ::grpcio::Result<super::feedapi::GetFeedResponse> {
        self.get_feed_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_feed_async_opt(&self, req: &super::feedapi::GetFeedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::GetFeedResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_GET_FEED, req, opt)
    }

    pub fn get_feed_async(&self, req: &super::feedapi::GetFeedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::GetFeedResponse>> {
        self.get_feed_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_feed_row_opt(&self, req: &super::feedapi::GetFeedRowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::GetFeedRowResponse> {
        self.client.unary_call(&METHOD_FEED_API_GET_FEED_ROW, req, opt)
    }

    pub fn get_feed_row(&self, req: &super::feedapi::GetFeedRowRequest) -> ::grpcio::Result<super::feedapi::GetFeedRowResponse> {
        self.get_feed_row_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_feed_row_async_opt(&self, req: &super::feedapi::GetFeedRowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::GetFeedRowResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_GET_FEED_ROW, req, opt)
    }

    pub fn get_feed_row_async(&self, req: &super::feedapi::GetFeedRowRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::GetFeedRowResponse>> {
        self.get_feed_row_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_contractor_opt(&self, req: &super::feedapi::CreateContractorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::CreateContractorResponse> {
        self.client.unary_call(&METHOD_FEED_API_CREATE_CONTRACTOR, req, opt)
    }

    pub fn create_contractor(&self, req: &super::feedapi::CreateContractorRequest) -> ::grpcio::Result<super::feedapi::CreateContractorResponse> {
        self.create_contractor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_contractor_async_opt(&self, req: &super::feedapi::CreateContractorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::CreateContractorResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_CREATE_CONTRACTOR, req, opt)
    }

    pub fn create_contractor_async(&self, req: &super::feedapi::CreateContractorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::CreateContractorResponse>> {
        self.create_contractor_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_contractor_opt(&self, req: &super::feedapi::UpdateContractorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::UpdateContractorResponse> {
        self.client.unary_call(&METHOD_FEED_API_UPDATE_CONTRACTOR, req, opt)
    }

    pub fn update_contractor(&self, req: &super::feedapi::UpdateContractorRequest) -> ::grpcio::Result<super::feedapi::UpdateContractorResponse> {
        self.update_contractor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_contractor_async_opt(&self, req: &super::feedapi::UpdateContractorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::UpdateContractorResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_UPDATE_CONTRACTOR, req, opt)
    }

    pub fn update_contractor_async(&self, req: &super::feedapi::UpdateContractorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::UpdateContractorResponse>> {
        self.update_contractor_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_contractors_opt(&self, req: &super::feedapi::GetContractorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::GetContractorsResponse> {
        self.client.unary_call(&METHOD_FEED_API_GET_CONTRACTORS, req, opt)
    }

    pub fn get_contractors(&self, req: &super::feedapi::GetContractorsRequest) -> ::grpcio::Result<super::feedapi::GetContractorsResponse> {
        self.get_contractors_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_contractors_async_opt(&self, req: &super::feedapi::GetContractorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::GetContractorsResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_GET_CONTRACTORS, req, opt)
    }

    pub fn get_contractors_async(&self, req: &super::feedapi::GetContractorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::GetContractorsResponse>> {
        self.get_contractors_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn have_seen_opt(&self, req: &super::feedapi::HaveSeenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::HaveSeenResponse> {
        self.client.unary_call(&METHOD_FEED_API_HAVE_SEEN, req, opt)
    }

    pub fn have_seen(&self, req: &super::feedapi::HaveSeenRequest) -> ::grpcio::Result<super::feedapi::HaveSeenResponse> {
        self.have_seen_opt(req, ::grpcio::CallOption::default())
    }

    pub fn have_seen_async_opt(&self, req: &super::feedapi::HaveSeenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::HaveSeenResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_HAVE_SEEN, req, opt)
    }

    pub fn have_seen_async(&self, req: &super::feedapi::HaveSeenRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::HaveSeenResponse>> {
        self.have_seen_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn seen_opt(&self, req: &super::feedapi::SeenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feedapi::SeenResponse> {
        self.client.unary_call(&METHOD_FEED_API_SEEN, req, opt)
    }

    pub fn seen(&self, req: &super::feedapi::SeenRequest) -> ::grpcio::Result<super::feedapi::SeenResponse> {
        self.seen_opt(req, ::grpcio::CallOption::default())
    }

    pub fn seen_async_opt(&self, req: &super::feedapi::SeenRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::SeenResponse>> {
        self.client.unary_call_async(&METHOD_FEED_API_SEEN, req, opt)
    }

    pub fn seen_async(&self, req: &super::feedapi::SeenRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feedapi::SeenResponse>> {
        self.seen_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait FeedApi {
    fn create_feed_row(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::CreateFeedRowRequest, sink: ::grpcio::UnarySink<super::feedapi::CreateFeedRowResponse>);
    fn add_state_to_feed_row(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::AddStateToFeedRowRequest, sink: ::grpcio::UnarySink<super::feedapi::AddStateToFeedRowResponse>);
    fn update_state(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::UpdateStateRequest, sink: ::grpcio::UnarySink<super::feedapi::UpdateStateResponse>);
    fn get_feed(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::GetFeedRequest, sink: ::grpcio::UnarySink<super::feedapi::GetFeedResponse>);
    fn get_feed_row(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::GetFeedRowRequest, sink: ::grpcio::UnarySink<super::feedapi::GetFeedRowResponse>);
    fn create_contractor(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::CreateContractorRequest, sink: ::grpcio::UnarySink<super::feedapi::CreateContractorResponse>);
    fn update_contractor(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::UpdateContractorRequest, sink: ::grpcio::UnarySink<super::feedapi::UpdateContractorResponse>);
    fn get_contractors(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::GetContractorsRequest, sink: ::grpcio::UnarySink<super::feedapi::GetContractorsResponse>);
    fn have_seen(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::HaveSeenRequest, sink: ::grpcio::UnarySink<super::feedapi::HaveSeenResponse>);
    fn seen(&mut self, ctx: ::grpcio::RpcContext, req: super::feedapi::SeenRequest, sink: ::grpcio::UnarySink<super::feedapi::SeenResponse>);
}

pub fn create_feed_api<S: FeedApi + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FEED_API_CREATE_FEED_ROW, move |ctx, req, resp| {
        instance.create_feed_row(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FEED_API_ADD_STATE_TO_FEED_ROW, move |ctx, req, resp| {
        instance.add_state_to_feed_row(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FEED_API_UPDATE_STATE, move |ctx, req, resp| {
        instance.update_state(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FEED_API_GET_FEED, move |ctx, req, resp| {
        instance.get_feed(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FEED_API_GET_FEED_ROW, move |ctx, req, resp| {
        instance.get_feed_row(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FEED_API_CREATE_CONTRACTOR, move |ctx, req, resp| {
        instance.create_contractor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FEED_API_UPDATE_CONTRACTOR, move |ctx, req, resp| {
        instance.update_contractor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FEED_API_GET_CONTRACTORS, move |ctx, req, resp| {
        instance.get_contractors(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_FEED_API_HAVE_SEEN, move |ctx, req, resp| {
        instance.have_seen(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_FEED_API_SEEN, move |ctx, req, resp| {
        instance.seen(ctx, req, resp)
    });
    builder.build()
}
