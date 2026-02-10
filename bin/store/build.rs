fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/proto/store.proto");

    tonic_build::configure()
        .build_server(true) // Enable server code (default)
        .build_client(false) // Enable client code (default)
        .out_dir("src/generated")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .message_attribute(".", "#[serde(default)]") // Apply serde(default) only to message types (structs)
        .protoc_arg("--experimental_allow_proto3_optional") // Enable proto3 optional fields
        .compile_protos(&["src/generated/proto/store.proto"], &["src/generated"])?;

    Ok(())
}
