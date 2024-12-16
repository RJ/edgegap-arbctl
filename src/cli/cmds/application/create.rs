use anyhow::Result;
use clap::Args;
use edgegap::{apis::applications_api::application_post, models::ApplicationPost};

use crate::{cli::Cmd, context::Ctx};

/// Creates a new application by reading a JSON payload from stdin
#[derive(Clone, Args)]
pub struct ArbctlApplicationCreate {
    /// Application name (must match name in payload)
    pub name: String,
}

impl Cmd for ArbctlApplicationCreate {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let payload = crate::payload_reader::read(ctx)?;
        let payload = serde_json::from_str::<ApplicationPost>(&payload)?;
        if payload.name != self.name {
            return Err(anyhow::anyhow!(
                "❌ Application name in json does not match command line argument"
            ));
        }
        match application_post(ctx.config(), payload) {
            Ok(_) => {
                println!("✅ Created '{}'", self.name);
                Ok(())
            }
            Err(e) => {
                eprintln!("CREATE APPLICATION RESPONSE: {:?}", e);
                Err(anyhow::anyhow!("❌ Failed to create application"))
            }
        }
    }
}
