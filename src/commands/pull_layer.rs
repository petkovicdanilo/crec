use std::path::PathBuf;

use crate::image::normalize_image_name;
use anyhow::Result;
use clap::Clap;
use indicatif::ProgressBar;
use oci_registry::Registry;

/// Pull layer with given digest to a disk
#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct PullLayer {
    /// Image name
    #[clap(name = "IMAGE_NAME", parse(from_str = normalize_image_name))]
    image_name: String,

    /// Layer digest
    digest: String,

    /// Output file
    #[clap(parse(from_os_str))]
    destination: PathBuf,
}

impl PullLayer {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        registry
            .pull_layer_with_progress_bar(
                &self.image_name,
                &self.digest,
                &self.destination,
                ProgressBar::new(0),
            )
            .await?;

        Ok(())
    }
}
