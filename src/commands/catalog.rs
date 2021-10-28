use anyhow::Result;
use clap::Clap;
use oci_registry::Registry;

/// List all images available in a registry
#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct Catalog {}

impl Catalog {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        let catalog = registry.catalog().await?;
        println!("{}", serde_json::to_string_pretty(&catalog)?);

        Ok(())
    }
}
