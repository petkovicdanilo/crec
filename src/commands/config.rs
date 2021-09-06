use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

#[derive(Clap, Debug)]
pub struct Config {
    /// Container image
    #[clap(name = "image")]
    pub image: String,

    /// Specific tag
    #[clap(name = "tag")]
    pub tag: String,
}

impl Config {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        // let manifest = registry.pull_manifest(&self.image, &self.tag).await?;
        // let digest = manifest.config().digest();

        // let configuration = registry.pull_configuration(&self.image, &digest).await?;
        // println!("{}", serde_json::to_string_pretty(&configuration)?);

        Ok(())
    }
}
