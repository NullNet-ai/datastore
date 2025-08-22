use crate::constants::paths;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={}", paths::proto::SOURCE_FILE);

    tonic_build::configure()
        .build_server(true) // Enable server code (default)
        .build_client(false) // Enable client code (default)
        .out_dir(paths::proto::GENERATED_DIR)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .message_attribute(".", "#[serde(default)]") // Apply serde(default) only to message types (structs)
        .compile_protos(&[paths::proto::SOURCE_FILE], &["src"])?;

    println!("cargo:warning=Successfully compiled proto files");
    Ok()
}
