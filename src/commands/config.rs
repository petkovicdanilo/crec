use anyhow::Result;
use clap::Clap;
use oci_registry::Registry;

use crate::image::{parse_image_id, ImageId};

/// Get a configuration object for given image
#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct Config {
    /// Image id
    #[clap(name = "IMAGE", parse(from_str = parse_image_id))]
    image_id: ImageId,
}

impl Config {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        let manifest = registry
            .pull_manifest(&self.image_id.name, &self.image_id.tag)
            .await?;
        let digest = manifest.config().digest();

        let configuration = registry
            .pull_configuration(&self.image_id.name, &digest)
            .await?;
        println!("{}", serde_json::to_string_pretty(&configuration)?);

        Ok(())
    }
}
