use clap_complete::{generate_to, shells::{Zsh,Bash}};
use clap::CommandFactory;
use std::env;
use std::io::Error;

include!("src/cli_parse.rs");

const COMMAND_NAME: &str = "cardauth";

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let zpath = generate_to(
        Zsh,
        &mut Cli::command(),
        COMMAND_NAME,
        outdir.clone(),
    )?;
    println!("cargo:warning=ZSH completion file is generated: {zpath:?}");
    let bpath = generate_to(
        Bash,
        &mut Cli::command(),
        COMMAND_NAME,
        outdir,
    )?;
    println!("cargo:warning=bash completion file is generated: {bpath:?}");


    Ok(())
}
