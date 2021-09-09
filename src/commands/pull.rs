use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use oci_registry::{
    oci_spec::image::{Arch, Os},
    registry::Registry,
};

use crate::platform::{parse_arch, parse_os, this_arch, this_os};

#[derive(Clap, Debug)]
pub struct Pull {
    /// Container image
    #[clap(name = "image")]
    image: String,

    /// Specific tag
    #[clap(name = "tag")]
    tag: String,

    /// Output folder
    #[clap(parse(from_os_str))]
    output: PathBuf,

    /// Os for which to pull an image.
    /// Value should be from those listed in the Go Language document
    /// for [`GOOS`](https://golang.org/doc/install/source#environment)
    #[clap(long, parse(try_from_str = parse_os))]
    os: Option<Os>,

    /// Architecture for which to pull an image.
    /// Value should be from those listed in the Go Language document
    /// for [`GOARCH`](https://golang.org/doc/install/source#environment)
    #[clap(long, parse(try_from_str = parse_arch))]
    arch: Option<Arch>,
}

impl Pull {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        let os = match &self.os {
            Some(os) => os.clone(),
            None => this_os(),
        };

        let arch = match &self.arch {
            Some(arch) => arch.clone(),
            None => this_arch(),
        };

        registry
            .pull_image(&self.image, &self.tag, &os, &arch, &self.output)
            .await?;

        Ok(())
    }
}
