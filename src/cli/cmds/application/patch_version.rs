use anyhow::Result;
use clap::Args;
use edgegap::{
    apis::applications_api::{app_version_get, app_versions_patch},
    models::{AppVersionPayload, AppVersionUpdatePayload},
};

use crate::{cli::Cmd, context::Ctx};

/// Patch application version (specify --patch args too)
#[derive(Clone, Args)]
pub struct ArbctlApplicationPatchVersion {
    /// Application name
    pub name: String,
    /// Version name
    pub version: String,
    /// Patches to apply to the application version json when sending PATCH request
    #[arg(long = "patch", num_args = 1.., number_of_values = 2, required = true)]
    pub patches: Option<Vec<String>>,
}

impl Cmd for ArbctlApplicationPatchVersion {
    fn run(&self, ctx: &mut Ctx) -> Result<()> {
        let Some(patches) = self.patches.as_ref() else {
            return Err(anyhow::anyhow!("No patches specified"));
        };
        // get existing version, and convert to json, apply patches
        let app = app_version_get(ctx.config(), &self.name, &self.version).unwrap();
        let mut json = serde_json::to_value(&app).unwrap();
        crate::patch::apply_patches(&mut json, patches)?;
        // convert patched json back to AppVersionUpdatePayload
        let mut payload: AppVersionUpdatePayload = serde_json::from_value(json).unwrap();
        // these fields apparenrly not allowed in the patch payload?
        // TODO this is an edgegap documentation+swagger bug?
        payload.req_cpu = None;
        payload.req_memory = None;
        payload.req_video = None;
        // ctx.api_config.base_path = "http://localhost:9999".to_string();
        match app_versions_patch(ctx.config(), &self.name, &self.version, payload) {
            Ok(_) => {
                println!("✅ Patched OK");
                Ok(())
            }
            Err(e) => {
                eprintln!("PATCH RESPONSE: {:?}", e);
                Err(anyhow::anyhow!("❌ Failed to patch application version"))
            }
        }
    }
}
