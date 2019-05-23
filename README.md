rust:
	$(PROTOC_CMD) --plugin=protoc-gen-rust-grpc=/Users/davidrivera/.cargo/bin/protoc-gen-rust-grpc --rust-grpc_out=build/rust --rust_out=build/rust proto/v1beta1/*.proto