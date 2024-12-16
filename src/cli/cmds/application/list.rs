use anyhow::Result;
use clap::Args;
use edgegap::{apis::applications_api::applications_get, models::Applications};
use serde::Serialize;
use serde_json::Value;

use crate::{cli::Cmd, context::Ctx, context::OutputFormat};

/// list targets
#[derive(Clone, Args)]
pub struct ArbctlApplicationList {
    /// only list installed targets
    #[arg(short, long)]
    pub show_disabled: bool,
}

impl Cmd for ArbctlApplicationList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let apps = applications_get(ctx.config()).unwrap();
        crate::renderer::render_thing(&apps, ctx);
        Ok(())
    }
}
