use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

use crate::image::{parse_image_id, ImageId};

/// Get manifest for given image
#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct Manifest {
    /// Image id
    #[clap(name = "IMAGE", parse(from_str = parse_image_id))]
    image_id: ImageId,
}

impl Manifest {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        let manifest = registry
            .pull_manifest(&self.image_id.name, &self.image_id.tag)
            .await?;
        println!("{}", serde_json::to_string_pretty(&manifest)?);

        Ok(())
    }
}
