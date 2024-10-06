use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=RUST_ANALYZER_CHECK");
    let result = SpirvBuilder::new("kernels", "spirv-unknown-spv1.3")
        .print_metadata(MetadataPrintout::Full)
        .build();
    if std::env::var("RUST_ANALYZER_CHECK").map(|env| env == "1").unwrap_or_default() {
        if let Err(msg) = result.map_err(|e| e.to_string()) {
            println!("cargo:warning={}", msg);
        }
        Ok(())
    } else {
        result.map(|_| ()).map_err(|e| Box::from(e))
    }
}
