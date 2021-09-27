use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

/// Pull layer with given digest to a disk
#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct PullLayer {
    /// Container image
    #[clap(name = "image")]
    image: String,

    /// Specific tag
    #[clap(name = "tag")]
    tag: String,

    /// Specific tag
    #[clap(name = "digest")]
    digest: String,

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
