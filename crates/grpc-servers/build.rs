// https://crates.io/crates/tonic-build
// https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
