mod list;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{cli::Cmd, context::Ctx};

/// Manage sessions
#[derive(Clone, Args)]
pub struct ArbctlDeployment {
    #[command(subcommand)]
    pub cmd: ArbctlDeploymentCmd,
}

#[enum_delegate::implement(Cmd)]
#[derive(Clone, Subcommand)]
pub enum ArbctlDeploymentCmd {
    List(list::ArbctlDeploymentList),
}

impl Cmd for ArbctlDeployment {
    fn update_ctx(&self, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }
    fn next_cmd(&self) -> Option<&dyn Cmd> {
        Some(&self.cmd)
    }
}
