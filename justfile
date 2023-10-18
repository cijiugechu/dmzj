alias b := build-proto
alias f := fmt

build-proto:
  GENERATE_PROTOBUF=1 cargo doc --no-deps

fmt:
  cargo fmt
  taplo format 