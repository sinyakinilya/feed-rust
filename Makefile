proto-install:
	cd src/proto/feedapi && \
	protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` feedapi.proto