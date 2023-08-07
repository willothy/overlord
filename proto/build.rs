pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("client.proto")?;
    tonic_build::compile_protos("server.proto")?;
    Ok(())
}
