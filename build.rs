extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/proto")
        .inputs(&["proto/feedapi.proto"])
        .include("proto")
        .run()
        .expect("Running protoc failed.");
}
