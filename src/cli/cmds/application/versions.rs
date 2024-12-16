use anyhow::Result;
use clap::Args;
use edgegap::apis::applications_api::app_versions_get;

use crate::{cli::Cmd, context::Ctx};

/// list targets
#[derive(Clone, Args)]
pub struct ArbctlApplicationVersions {
    /// Application name
    pub name: String,
}

impl Cmd for ArbctlApplicationVersions {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let app = app_versions_get(ctx.config(), &self.name).unwrap();
        crate::renderer::render_thing(&app, ctx);
        Ok(())
    }
}
