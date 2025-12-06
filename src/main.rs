use std::process::exit;
use std::io;
use std::io::prelude::*;
use clap::{Parser,CommandFactory};
use clap_complete::generate;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordVerifier, PasswordHasher, SaltString
    },
    Argon2
};
use tokio::{select, time::{Duration, sleep}};
use toml::Table;

mod cli_parse;
use cli_parse::*;
mod msr605x;
use msr605x::*;

#[tokio::main]
async fn main() {
    let mut args = Cli::parse();
    if let Commands::Completions { command_name, shell } = &args.command {
        generate(shell.clone(), &mut Cli::command(),
            command_name.clone().unwrap_or(std::env::args().next().unwrap()),
            &mut std::io::stdout());
        std::process::exit(0);
    }

    if let Commands::Auth {exit_on_pwd: true, ..} = &args.command {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        if input.len() != 1 || input[0] != 0x43 {
            exit(2);
        } // if actual password provided, do something else
    }
    let mut msr = MSR605x::new().await.unwrap();
    msr.reset().await.unwrap();

    let mut rcvd = Vec::new();
    let mut errctr = 0;
    select!(
        _ = async {
            loop {
                msr.cmd(b"\x1br").await.unwrap();
                rcvd = msr.receive().await.unwrap();
                if errctr >= args.retry_count || rcvd.ends_with(b"\x1b0") {
                    break
                }
                errctr += 1;
            }
        } => {},
        _ = sleep(Duration::from_secs(args.timeout)) => {
            msr.reset().await.unwrap();
            msr.end().await.unwrap();
            exit(4);
        }
    );
    if !rcvd.ends_with(b"\x1b0") {
        println!("Cannot read valid data from card.");
        exit(3);
    }

    // FIXME: allow non-default options
    let argon2 = Argon2::default();
    if let Commands::Read {print, hash} = &args.command {
        if *print {
            println!("{:?}", String::from_utf8_lossy(&rcvd));
        }
        if *hash {
            let salt = SaltString::generate(&mut OsRng);
            println!("{}",
                argon2.hash_password(&rcvd, &salt).unwrap().to_string());
        }
    }
    if let Commands::Auth { exit_on_pwd: _, user, users_file } = &mut args.command {
        let mut users_contents = String::new();
        users_file.read_to_string(&mut users_contents).unwrap();
        let users = users_contents.parse::<Table>().unwrap();
        let hash = match users.get(user) {
            Some(x) => PasswordHash::new(x.as_str().unwrap()).unwrap(),
            None => {exit(1)}
        };
        match argon2.verify_password(&rcvd, &hash) {
            Ok(()) => {
                println!("Success");
                exit(0);
            },
            Err(argon2::password_hash::Error::Password) => {
                println!("Failure");
                exit(1);
            },
            Err(_) => exit(1),
        };
    }

    msr.end().await.unwrap();
}
