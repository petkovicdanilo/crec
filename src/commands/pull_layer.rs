use std::path::PathBuf;

use anyhow::Result;
use oci_registry::registry::Registry;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct PullLayer {
    /// Container image
    #[structopt(name = "image")]
    pub image: String,

    /// Specific tag
    #[structopt(name = "tag")]
    pub tag: String,

    /// Specific tag
    #[structopt(name = "digest")]
    pub digest: String,

    /// Output file
    #[structopt(parse(from_os_str))]
    destination: PathBuf,
}

impl PullLayer {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        // registry
        //     .pull_blob(&self.image, &self.digest, &self.destination)
        //     .await?;

        Ok(())
    }
}
