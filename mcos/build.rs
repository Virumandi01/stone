fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This tells Rust to look in the shared folder and compile our gRPC rules
    tonic_build::compile_protos("../shared/stone.proto")?;
    Ok(())
}