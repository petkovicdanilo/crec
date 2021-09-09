use anyhow::Result;
use oci_registry::oci_spec::image::{Arch, Os};

pub fn parse_arch(src: &str) -> Result<Arch> {
    Ok(src.into())
}

pub fn parse_os(src: &str) -> Result<Os> {
    Ok(src.into())
}

/// Values took from [docs](https://doc.rust-lang.org/std/env/consts/constant.ARCH.html)
pub fn this_arch() -> Arch {
    match std::env::consts::ARCH {
        "x86" => Arch::i386,
        "x86_64" => Arch::Amd64,
        "arm" => Arch::ARM,
        "aarch64" => Arch::ARM64,
        "mips" => Arch::Mips,
        "mips64" => Arch::Mips64,
        "powerpc" => Arch::PowerPC,
        "powerpc64" => Arch::PowerPC64,
        "riscv64" => Arch::RISCV64,
        "s390x" => Arch::s390x,
        "sparc64" => Arch::SPARC64,
        other => panic!("Unknown architecture: '{}'", other),
    }
}

/// Values took from [docs](https://doc.rust-lang.org/std/env/consts/constant.OS.html)
pub fn this_os() -> Os {
    match std::env::consts::OS {
        "linux" => Os::Linux,
        "macos" => Os::Darwin,
        "ios" => Os::iOS,
        "freebsd" => Os::FreeBSD,
        "dragonfly" => Os::DragonFlyBSD,
        "netbsd" => Os::NetBSD,
        "openbsd" => Os::OpenBSD,
        "solaris" => Os::Solaris,
        "android" => Os::Android,
        "windows" => Os::Windows,
        other => panic!("Unknown os '{}'", other),
    }
}
