use anyhow::Result;
use clap::Parser;

mod cli;
mod context;
mod patch;
mod payload_reader;
mod renderer;

use crate::cli::{Arbctl, Cmd};

fn main() -> Result<()> {
    let args = Arbctl::parse();
    let mut ctx = context::Ctx::default();
    let args: &dyn Cmd = &args;
    args.walk_exec(&mut ctx)
}
