use anyhow::Result;
use clap::Args;
use edgegap::{
    apis::applications_api::{app_version_get, app_versions_patch},
    models::{AppVersionPayload, AppVersionUpdatePayload},
};

use crate::{cli::Cmd, context::Ctx};

/// Get application version
#[derive(Clone, Args)]
pub struct ArbctlApplicationVersion {
    /// Application name
    pub name: String,
    /// Version name
    pub version: String,
}

impl Cmd for ArbctlApplicationVersion {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let app = app_version_get(ctx.config(), &self.name, &self.version).unwrap();
        crate::renderer::render_thing(&app, ctx);
        Ok(())
    }
}
