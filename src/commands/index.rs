use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

use crate::image::{parse_image_id, ImageId};

/// Get index object for given image
#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct Index {
    /// Image id
    #[clap(name = "IMAGE", parse(from_str = parse_image_id))]
    image_id: ImageId,
}

impl Index {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        let index = registry
            .pull_index(&self.image_id.name, &self.image_id.tag)
            .await?;
        println!("{}", serde_json::to_string_pretty(&index)?);

        Ok(())
    }
}
