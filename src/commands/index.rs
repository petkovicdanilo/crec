use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

#[derive(Clap, Debug)]
pub struct Index {
    /// Container image
    #[clap(name = "image")]
    pub image: String,

    /// Specific tag
    #[clap(name = "tag")]
    pub tag: String,
}

impl Index {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        let index = registry.pull_index("library/ubuntu", "latest").await?;
        println!("{}", serde_json::to_string_pretty(&index)?);

        Ok(())
    }
}
