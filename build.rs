use std::{env, error::Error, path::PathBuf};
use g2h::BridgeGenerator;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("calculator_descriptor.bin"))
        .compile_protos(&["proto/calculator.proto"], &["proto"])?;
    
    tonic_build::compile_protos("proto/calculator.proto")?;

    BridgeGenerator::with_tonic_build()
        .build_prost_config()
        .compile_protos(&["proto/calculator.proto"], &["proto"])?;

    Ok(())
}
