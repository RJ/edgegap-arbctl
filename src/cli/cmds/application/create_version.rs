use anyhow::Result;
use clap::Args;
use edgegap::{apis::applications_api::app_version_post, models::AppVersionPayload};

use crate::{cli::Cmd, context::Ctx};

/// list targets
#[derive(Clone, Args)]
pub struct ArbctlApplicationCreateVersion {
    /// Application name
    pub name: String,
    /// Version name
    pub version: String,
}

impl Cmd for ArbctlApplicationCreateVersion {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let payload = crate::payload_reader::read(ctx)?;
        let payload = serde_json::from_str::<AppVersionPayload>(&payload)?;
        // verify the version we think we're creating matches the json
        if payload.name != self.version {
            return Err(anyhow::anyhow!(
                "Version name in json does not match command line argument"
            ));
        }
        // ctx.api_config.base_path = "http://localhost:9999".to_string();
        match app_version_post(ctx.config(), &self.name, payload) {
            Ok(_) => {
                println!("✅ Created '{}' version '{}'", self.name, self.version);
                Ok(())
            }
            Err(e) => {
                eprintln!("CREATE VERSION RESPONSE: {:?}", e);
                Err(anyhow::anyhow!("❌ Failed to create application version"))
            }
        }
    }
}
