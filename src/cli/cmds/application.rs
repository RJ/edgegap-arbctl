mod create;
mod create_version;
mod get;
mod list;
mod patch_version;
mod version;
mod versions;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::{cli::Cmd, context::Ctx};

/// Manage applications
#[derive(Clone, Args)]
pub struct ArbctlApplication {
    /// List applications
    #[command(subcommand)]
    pub cmd: ArbctlApplicationCmd,
}

#[enum_delegate::implement(Cmd)]
#[derive(Clone, Subcommand)]
pub enum ArbctlApplicationCmd {
    List(list::ArbctlApplicationList),
    Get(get::ArbctlApplicationGet),
    Versions(versions::ArbctlApplicationVersions),
    Version(version::ArbctlApplicationVersion),
    PatchVersion(patch_version::ArbctlApplicationPatchVersion),
    CreateVersion(create_version::ArbctlApplicationCreateVersion),
    Create(create::ArbctlApplicationCreate),
}

impl Cmd for ArbctlApplication {
    fn update_ctx(&self, _ctx: &mut Ctx) -> Result<()> {
        // ctx.target_toolchain = Some(self.toolchain.clone());
        Ok(())
    }
    fn next_cmd(&self) -> Option<&dyn Cmd> {
        Some(&self.cmd)
    }
}
