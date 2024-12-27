fn main() {
    let mut config = prost_build::Config::new();
    config
        .out_dir("src/message")
        .compile_protos(&["cluster_event.proto", "cluster_data.proto"], &["proto/"])
        .unwrap();
}
