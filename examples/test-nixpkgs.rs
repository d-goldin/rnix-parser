#![feature(rust_2018_preview)]

#[macro_use]
extern crate failure;

use failure::Error;
use std::{env, fs, path::Path};

fn main() -> Result<(), Error> {
    let path = env::var("NIX_PATH")?;
    let nixpkgs = path.split(':')
        .find(|s| s.starts_with("nixpkgs="))
        .ok_or_else(|| format_err!("no store path found"))?;

    println!("Nix store path: {}", nixpkgs);

    recurse(Path::new(&nixpkgs["nixpkgs=".len()..]))
}
fn recurse(path: &Path) -> Result<(), Error> {
    if path.metadata()?.is_file() {
        if path.extension().and_then(|s| s.to_str()) != Some("nix") {
            return Ok(());
        }
        println!("Checking {}", path.display());
        let original = fs::read_to_string(path)?;
        let parsed = rnix::parse(&original)?.to_string();
        if original != parsed {
            eprintln!("Original input does not match parsed output!");
            println!("Input:");
            println!("----------");
            println!("{}", original);
            println!("----------");
            println!("Output:");
            println!("----------");
            println!("{}", parsed);
            println!("----------");
            bail!("parsing error");
        }
        return Ok(());
    } else {
        for entry in path.read_dir()? {
            recurse(&entry?.path())?;
        }
    }
    Ok(())
}