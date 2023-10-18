alias b := build-proto

build-proto:
  GENERATE_PROTOBUF=1 cargo doc --no-deps