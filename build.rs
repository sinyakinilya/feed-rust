extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/proto/feedapi/")
        .inputs(&["src/proto/feedapi/feedapi.proto"])
        .include("src/proto")
        .run()
        .expect("Running protoc failed.");
}
