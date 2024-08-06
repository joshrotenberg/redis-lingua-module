#[cfg(not(feature = "integration-tests"))]
fn main() {}

#[cfg(feature = "integration-tests")]
use std::{env, process::Command};

#[cfg(feature = "integration-tests")]
use anyhow::{bail, Result};

#[cfg(feature = "integration-tests")]
fn main() -> Result<()> {
    let cwd = env::var("CARGO_MANIFEST_DIR")?;
    let dockerfile = format!("{cwd}/Dockerfile");

    println!("cargo:warning={}", "Building from '{dockerfile}'");
    let output = Command::new("docker")
        .arg("build")
        .arg("--file")
        .arg(dockerfile)
        .arg("--force-rm")
        .arg("--tag")
        .arg("redis_lingua:latest")
        .arg(".")
        .output()?;

    if !output.status.success() {
        eprintln!("stderr: {}", String::from_utf8(output.stderr)?);
        bail!("unable to build redis_lingua:latest");
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Dockerfile");

    Ok(())
}
