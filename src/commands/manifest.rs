use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

#[derive(Clap, Debug)]
pub struct Manifest {
    /// Container image
    #[clap(name = "image")]
    image: String,

    /// Specific tag
    #[clap(name = "tag")]
    tag: String,
}

impl Manifest {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        let manifest = registry.pull_manifest(&self.image, &self.tag).await?;
        println!("{}", serde_json::to_string_pretty(&manifest)?);

        Ok(())
    }
}
