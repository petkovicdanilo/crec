use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

#[derive(Clap, Debug)]
pub struct PullLayer {
    /// Container image
    #[clap(name = "image")]
    pub image: String,

    /// Specific tag
    #[clap(name = "tag")]
    pub tag: String,

    /// Specific tag
    #[clap(name = "digest")]
    pub digest: String,

    /// Output file
    #[clap(parse(from_os_str))]
    destination: PathBuf,
}

impl PullLayer {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        registry
            .pull_blob(&self.image, &self.digest, &self.destination)
            .await?;

        Ok(())
    }
}
