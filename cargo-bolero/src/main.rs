use crate::{
    config::Config,
    fuzz::{Fuzz, FuzzArgs},
    new::New,
    shrink::{Shrink, ShrinkArgs},
};
use failure::Error;
use structopt::StructOpt;

mod afl;
mod config;
mod fuzz;
mod fuzzer;
mod libfuzzer;
mod manifest;
mod new;
mod shrink;

#[derive(Debug, StructOpt)]
#[allow(clippy::large_enum_variant)]
enum Commands {
    Fuzz(Fuzz),
    Shrink(Shrink),
    New(New),
}

impl Commands {
    fn exec(&self) -> Result<(), Error> {
        match self {
            Self::Fuzz(cmd) => cmd.exec(),
            Self::Shrink(cmd) => cmd.exec(),
            Self::New(cmd) => cmd.exec(),
        }
    }
}

const DEFAULT_TARGET: &str = env!("DEFAULT_TARGET");

fn main() {
    let args = std::env::args()
        .enumerate()
        .filter_map(|(i, v)| match (i, v.as_ref()) {
            (1, "bolero") => None, // cargo passes the subcommand so filter it out
            _ => Some(v),
        });

    if let Err(err) = Commands::from_iter(args).exec() {
        eprintln!("error: {}", err);
        std::process::exit(1);
    }
}

pub(crate) fn exec(mut cmd: std::process::Command) -> Exit {
    let status = cmd
        .spawn()
        .expect("failed to start fuzzer")
        .wait()
        .expect("fuzzer was not running");

    Exit(status.code().unwrap_or(0))
}

#[derive(Debug)]
pub(crate) struct Exit(i32);

impl Exit {
    fn exit_on_error(self) {
        let code = self.0;
        if code != 0 {
            std::process::exit(code);
        }
    }
}
