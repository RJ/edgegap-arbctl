use anyhow::Result;
use clap::Args;
use edgegap::apis::applications_api::application_get;

use crate::{cli::Cmd, context::Ctx};

/// list targets
#[derive(Clone, Args)]
pub struct ArbctlApplicationGet {
    /// Application name
    pub name: String,
}

impl Cmd for ArbctlApplicationGet {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let app = application_get(ctx.config(), &self.name).unwrap();
        crate::renderer::render_thing(&app, ctx);
        Ok(())
    }
}
