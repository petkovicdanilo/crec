use std::path::PathBuf;

use anyhow::Result;
use oci_registry::registry::Registry;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Pull {
    /// Container image
    #[structopt(name = "image")]
    pub image: String,

    /// Specific tag
    #[structopt(name = "tag")]
    pub tag: String,

    /// Output folder
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}

impl Pull {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        // registry
        //     .pull_image(&self.image, &self.tag, &self.output)
        //     .await?;

        Ok(())
    }
}
