fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Write generated code to src/generated/ so you can inspect it. Run `cargo build -p counter-service` to generate.
    std::fs::create_dir_all("src/generated").ok();
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/generated")
        .compile_protos(&["proto/code_service.proto"], &["proto"])?;
    Ok(())
}
