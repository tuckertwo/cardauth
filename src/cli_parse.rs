use clap::{Parser,Subcommand};
use clap_complete::Shell;
use clio::Input;

#[derive(Parser)]
#[command(version, about, long_about = None, infer_subcommands = true)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, long, default_value="3")]
    pub retry_count: u64,

    #[arg(short, long, default_value="10")]
    pub timeout: u64,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Completions {
        #[arg(short, long)]
        command_name: Option<String>,

        #[arg(value_enum, default_value="zsh")]
        shell: Shell,
    },
    Read {
        #[arg(short, long, default_value="true",
            action = clap::ArgAction::Set,
            value_parser = clap::builder::BoolishValueParser::new())]
        print: bool,
        #[arg(short='H', long, default_value="true",
            action = clap::ArgAction::Set,
            value_parser = clap::builder::BoolishValueParser::new())]
        hash: bool,
    },
    Auth {
        #[arg(short, long, default_value="users.toml")]
        users_file: Input,

        #[arg(short, long, default_value="true",
            action = clap::ArgAction::Set,
            value_parser = clap::builder::BoolishValueParser::new())]
        exit_on_pwd: bool,

        #[arg(env="PAM_USER")]
        user: String,
    },
}
