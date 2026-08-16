fn main() {
    // ITStore: these are read with option_env! in src/config.rs, and Cargo does not track
    // env vars used that way. Without this, a cached CI build keeps the previously baked
    // values when the workflow env / secrets change.
    for k in [
        "ITSTORE_CONN_TYPE",
        "ITSTORE_PRESET_PASSWORD",
        "ITSTORE_PRESET_SALT",
    ] {
        println!("cargo:rerun-if-env-changed={}", k);
    }

    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
}
