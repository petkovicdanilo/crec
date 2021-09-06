use anyhow::Result;
use oci_registry::registry::Registry;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Manifest {
    /// Container image
    #[structopt(name = "image")]
    pub image: String,

    /// Specific tag
    #[structopt(name = "tag")]
    pub tag: String,
}

impl Manifest {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        let manifest = registry.pull_manifest(&self.image, &self.tag).await?;
        println!("{}", serde_json::to_string_pretty(&manifest)?);

        Ok(())
    }
}
