extern crate protobuf_codegen;

fn main() {
    let customize = protobuf_codegen::Customize::default().tokio_bytes(true);

    protobuf_codegen::Codegen::new()
        .customize(customize)
        .pure()
        .out_dir("src/protos")
        .inputs(["protos/details.proto"])
        .include("protos")
        .run()
        .expect("Codegen failed.");
}
