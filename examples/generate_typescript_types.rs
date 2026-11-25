//! This example exports meshtastic/protobufs Rust types into TypeScript
//!
//! Run this example with the command `cargo run --example generate_typescript_types --features "ts-gen"`
extern crate meshtastic;

use specta::Format as _;
use specta_typescript::semantic::Configuration;
use specta_typescript::Typescript;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Custom mapping for BigInt
    let types = specta_serde::Format.map_types(&specta::collect())?;
    let types = Configuration::empty()
        .enable_lossless_bigints()
        .apply_types(&types);

    // Exports relative to the root workspace directory
    Typescript::default()
        .layout(specta_typescript::Layout::ModulePrefixedName)
        .export_to("./examples/bindings.ts", &types, specta_serde::Format)?;

    Ok(())
}
