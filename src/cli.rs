mod cmds;

use anyhow::Result;
use clap::{Parser, Subcommand};
use edgegap::apis::configuration::*;

use crate::{
    cli::cmds::*,
    context::{Ctx, OutputFormat},
};

/// not rustup
#[derive(Parser)]
pub struct Arbctl {
    /// Edgegap api key, also read from env var EDGEGAP_API_KEY
    #[arg(long)]
    pub api_key: Option<String>,

    #[arg(long, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,

    /// Strategically omit annoyingly large fields like base64 encoded images
    #[arg(long, default_value_t = false)]
    pub pretty_redactions: bool,

    /// Json pointer to extract from the response json
    #[arg(long)]
    pub json_pointer: Option<String>,

    /// Patches to apply to the response json before printing
    /// (this does not apply to any PATCH requests sent to the API)
    #[arg(long = "render-patch", num_args = 1.., number_of_values = 2)]
    pub render_patches: Option<Vec<String>>,

    #[command(subcommand)]
    pub cmd: ArbctlCmd,
}

#[enum_delegate::implement(Cmd)]
#[derive(Clone, Subcommand)]
pub enum ArbctlCmd {
    Application(application::ArbctlApplication),
    Deployment(deployment::ArbctlDeployment),
}

impl Cmd for Arbctl {
    fn update_ctx(&self, ctx: &mut Ctx) -> Result<()> {
        if let Some(key) = self.api_key.as_ref() {
            ctx.api_config.api_key = Some(ApiKey {
                prefix: None,
                key: key.clone(),
            });
        } else if ctx.api_config.api_key.is_none() {
            return Err(anyhow::anyhow!(
                "Edgegap API key is required. Set with --api-key or env var EDGEGAP_API_KEY"
            ));
        }

        ctx.format = self.format;
        ctx.render_patches = self.render_patches.clone();
        ctx.pretty_redactions = self.pretty_redactions;
        ctx.json_pointer = self.json_pointer.clone();
        Ok(())
    }
    fn next_cmd(&self) -> Option<&dyn Cmd> {
        Some(&self.cmd)
    }
}

#[enum_delegate::register]
pub trait Cmd {
    fn update_ctx(&self, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }

    fn run(&self, _ctx: &mut Ctx) -> Result<()> {
        Ok(())
    }

    fn next_cmd(&self) -> Option<&dyn Cmd> {
        None
    }
}

impl<'a> dyn Cmd + 'a {
    pub fn walk_exec(&self, ctx: &mut Ctx) -> Result<()> {
        self.update_ctx(ctx)?;
        self.run(ctx)?;
        if let Some(next) = self.next_cmd() {
            return next.walk_exec(ctx);
        }

        Ok(())
    }
}
