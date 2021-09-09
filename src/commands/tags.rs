use anyhow::Result;
use clap::Clap;
use oci_registry::registry::Registry;

#[derive(Clap, Debug)]
pub struct Tags {
    /// Container image
    #[clap(name = "image")]
    image: String,
}

impl Tags {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        let tags = registry.list_tags(&self.image).await?;
        println!("{}", serde_json::to_string_pretty(&tags)?);

        Ok(())
    }
}
