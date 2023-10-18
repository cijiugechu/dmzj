extern crate protobuf_codegen;

fn main() {
    if std::env::var_os("GENERATE_PROTOBUF").is_some() {
        let customize =
            protobuf_codegen::Customize::default().tokio_bytes(true);

        protobuf_codegen::Codegen::new()
            .customize(customize)
            .pure()
            .out_dir("src/protos")
            .inputs(["protos/comic.proto"])
            .include("protos")
            .run()
            .expect("Codegen failed.");
    }
}
