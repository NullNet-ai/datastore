fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/proto/store.proto");

    tonic_build::configure()
        .build_server(true) // Enable server code (default)
        .build_client(false) // Enable client code (default)
        .out_dir("src/generated")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(default)]") // Custom output directory
        .compile_protos(&["src/proto/store.proto"], &["src"])?;

    println!("cargo:warning=Successfully compiled proto files");
    Ok(())
}
