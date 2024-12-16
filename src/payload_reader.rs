use crate::context::{Ctx, PayloadSource};
use anyhow::Result;
use std::io::Read;

/// Read json payload from whatever source.
pub fn read(ctx: &Ctx) -> Result<String> {
    let result = match ctx.payload_source {
        PayloadSource::Stdin => stdin(),
    }?;
    // Verify the input is valid JSON
    if serde_json::from_str::<serde_json::Value>(&result).is_ok() {
        return Ok(result);
    }
    Err(anyhow::anyhow!(
        "Invalid JSON from {:?}",
        ctx.payload_source
    ))
}

pub fn stdin() -> Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
