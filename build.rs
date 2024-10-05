use spirv_builder::{MetadataPrintout, SpirvBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SpirvBuilder::new("kernels", "spirv-unknown-vulkan1.1")
        .print_metadata(MetadataPrintout::None)
        .build()?;
    Ok(())
}
