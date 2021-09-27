use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

/// Get index object for given image
#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct Index {
    /// Container image
    #[clap(name = "image")]
    image: String,

    /// Specific tag
    #[clap(name = "tag")]
    tag: String,
}

impl Index {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        let index = registry.pull_index("library/ubuntu", "latest").await?;
        println!("{}", serde_json::to_string_pretty(&index)?);

        Ok(())
    }
}
