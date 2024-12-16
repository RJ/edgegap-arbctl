use anyhow::Result;
use clap::Args;
use edgegap::apis::deployments_api::deployments_get;

use crate::{cli::Cmd, context::Ctx};

/// list targets
#[derive(Clone, Args)]
pub struct ArbctlDeploymentList {
    /// A query filter to pass to the deployments api
    pub query: Option<String>,
}

impl Cmd for ArbctlDeploymentList {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let deployments = deployments_get(ctx.config(), self.query.as_deref())?;
        println!("{:#?}", deployments);
        Ok(())
    }
}
