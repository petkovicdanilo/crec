mod commands;

use std::str::FromStr;

use anyhow::Result;
use clap::Clap;
use commands::{config, index, manifest, pull, pull_layer, tags};
use oci_registry::registry::Registry;

#[derive(Clap, Debug)]
enum KnownRegistry {
    Docker,
    Quay,
    Mcr,
}

impl FromStr for KnownRegistry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "docker" => Ok(KnownRegistry::Docker),
            "quay" => Ok(KnownRegistry::Quay),
            "mcr" => Ok(KnownRegistry::Mcr),
            _ => Err("no match"),
        }
    }
}

/// Container Registry Client -
/// command line utility to communicate with OCI registries
#[derive(Clap, Debug)]
#[clap(name = "crec")]
struct Opt {
    /// Registry to work with. Possible values:
    /// docker, quay, mcr
    #[clap(long, default_value = "docker")]
    registry: KnownRegistry,

    /// Url of the registry without https:// in front.
    /// For example: registry-1.docker.io
    #[clap(long, conflicts_with = "registry")]
    registry_url: Option<String>,

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

fn known_registry_url(registry: &KnownRegistry) -> String {
    match registry {
        KnownRegistry::Docker => String::from("https://registry-1.docker.io"),
        KnownRegistry::Quay => String::from("https://quay.io"),
        KnownRegistry::Mcr => String::from("https://mcr.microsoft.com"),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();
    let registry_url = match opt.registry_url {
        Some(registry_url) => format!("https://{}", registry_url),
        None => known_registry_url(&opt.registry),
    };
    let registry = Registry::new(&registry_url);

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
