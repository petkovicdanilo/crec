mod commands;

use anyhow::Result;
use clap::Clap;
use commands::{config, index, manifest, pull, pull_layer, tags};
use oci_registry::registry::{Registry, RegistryType};

/// Container Registry Client -
/// command line utility to communicate with OCI registries
#[derive(Clap, Debug)]
#[clap(name = "crec")]
struct Opt {
    /// Registry to work with
    #[clap(long, default_value = "docker")]
    registry: String,

    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clap, Debug)]
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
    let opt = Opt::parse();
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
