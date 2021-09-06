use anyhow::Result;
use oci_registry::registry::Registry;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Tags {
    /// Container image
    #[structopt(name = "image")]
    pub image: String,
}

impl Tags {
    pub async fn exec(&self, registry: Registry) -> Result<()> {
        // let tags = registry.list_tags(&self.image).await?;
        // println!("{}", serde_json::to_string_pretty(&tags)?);

        Ok(())
    }
}
