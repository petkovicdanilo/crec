use anyhow::Result;
use oci_registry::registry::Registry;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Index {
    /// Container image
    #[structopt(name = "image")]
    pub image: String,

    /// Specific tag
    #[structopt(name = "tag")]
    pub tag: String,
}

impl Index {
    pub async fn exec(&self, mut registry: Registry) -> Result<()> {
        let index = registry.pull_index("library/ubuntu", "latest").await?;
        println!("{}", serde_json::to_string_pretty(&index)?);

        Ok(())
    }
}
