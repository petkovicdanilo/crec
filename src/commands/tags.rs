use crate::image::normalize_image_name;
use anyhow::Result;
use clap::Clap;
use oci_registry::Registry;

/// List all tags available for a given image
#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct Tags {
    /// Image name
    #[clap(name = "IMAGE_NAME", parse(from_str = normalize_image_name))]
    image_name: String,
}

impl Tags {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        let tags = registry.list_tags(&self.image_name).await?;
        println!("{}", serde_json::to_string_pretty(&tags)?);

        Ok(())
    }
}
