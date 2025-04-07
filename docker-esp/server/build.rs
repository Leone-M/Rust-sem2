use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/esp_data.proto"], &["src/"])?;
    Ok(())
}