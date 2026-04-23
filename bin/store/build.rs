fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/generated/proto/store.proto");
    println!("cargo:rerun-if-changed=proto/code_service.proto");

    // Store gRPC (server + types)
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .out_dir("src/generated")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute(".", "#[allow(dead_code)]")
        .message_attribute(".", "#[serde(default)]")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["src/generated/proto/store.proto"], &["src/generated"])?;

    // Code service client only (external service; proto outside generated).
    // Output to src/ so store-generator (which resets generated/) never overwrites this.
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src")
        .compile_protos(&["proto/code_service.proto"], &["proto"])?;

    Ok(())
}
