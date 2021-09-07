use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

#[derive(Clap, Debug)]
pub struct Pull {
    /// Container image
    #[clap(name = "image")]
    pub image: String,

    /// Specific tag
    #[clap(name = "tag")]
    pub tag: String,

    /// Output folder
    #[clap(parse(from_os_str))]
    output: PathBuf,
}

impl Pull {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        registry
            .pull_image(&self.image, &self.tag, &self.output)
            .await?;

        Ok(())
    }
}
