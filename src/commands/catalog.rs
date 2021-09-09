use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

#[derive(Clap, Debug)]
pub struct Catalog {}

impl Catalog {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        let catalog = registry.catalog().await?;
        println!("{}", serde_json::to_string_pretty(&catalog)?);

        Ok(())
    }
}
