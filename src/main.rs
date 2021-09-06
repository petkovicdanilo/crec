mod commands;

use anyhow::Result;
use commands::{config, index, manifest, pull, pull_layer, tags};
use oci_registry::registry::{Registry, RegistryType};
use structopt::StructOpt;

/// Container Registry Client -
/// command line utility to communicate with OCI registries
#[derive(StructOpt, Debug)]
#[structopt(name = "crec")]
struct Opt {
    /// Registry to work with
    #[structopt(long, default_value = "docker")]
    registry: String,

    #[structopt(flatten)]
    subcommand: SubCommand,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    Manifest(manifest::Manifest),
    Index(index::Index),
    Tags(tags::Tags),
    Config(config::Config),
    PullLayer(pull_layer::PullLayer),
    Pull(pull::Pull),
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    let registry = Registry::new(RegistryType::Docker);

    match opt.subcommand {
        SubCommand::Manifest(manifest) => manifest.exec(registry).await?,
        SubCommand::Index(index) => index.exec(registry).await?,
        SubCommand::Tags(tags) => tags.exec(registry).await?,
        SubCommand::Config(config) => config.exec(registry).await?,
        SubCommand::PullLayer(pull_layer) => pull_layer.exec(registry).await?,
        SubCommand::Pull(pull) => pull.exec(registry).await?,
    };

    Ok(())
}
