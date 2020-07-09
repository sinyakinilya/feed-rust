CONSUL_ADDR=localhost:8520

proto-install:
	cd src/proto/feedapi && \
	protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` feedapi.proto

run-docker:
	docker-compose -f deployments/docker-compose.yml up --build -d

down-docker:
	docker-compose -f deployments/docker-compose.yml down

install-config-sync:
	go install  github.com/dmarket/config-sync

local-consul-config-export:
	config-sync i ./configs/consul-config-local.json --addr $(CONSUL_ADDR)
