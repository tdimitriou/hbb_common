fn main() {
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

    // Optional self-hosted hbbs: set RENDEZVOUS_SERVER and RS_PUB_KEY in the environment
    // at compile time (e.g. GitHub Actions secrets mapped to env in workflows).
    println!("cargo:rerun-if-env-changed=RENDEZVOUS_SERVER");
    println!("cargo:rerun-if-env-changed=RS_PUB_KEY");
    if let Ok(v) = std::env::var("RENDEZVOUS_SERVER") {
        if !v.is_empty() {
            println!("cargo:rustc-env=RENDEZVOUS_SERVER={v}");
        }
    }
    if let Ok(v) = std::env::var("RS_PUB_KEY") {
        if !v.is_empty() {
            println!("cargo:rustc-env=RS_PUB_KEY={v}");
        }
    }
}
