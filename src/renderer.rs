use crate::patch::apply_patches;
use serde::Serialize;
use serde_json::Value;

use crate::context::{Ctx, OutputFormat};

pub fn render_thing(thing: &impl Serialize, ctx: &Ctx) {
    let mut json = serde_json::to_value(thing).unwrap();

    if let Some(patches) = &ctx.render_patches {
        apply_patches(&mut json, patches).expect("Failed to apply patches");
    }

    if let Some(pointer) = &ctx.json_pointer {
        json = json
            .pointer(pointer)
            .cloned()
            .unwrap_or_else(|| panic!("Pointer {pointer} not found in json"));
    }

    if ctx.pretty_redactions {
        redact_fields(&mut json, ctx);
    }

    match ctx.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
        OutputFormat::Text => render_as_text(json, ctx),
    }
}

fn render_as_text(thing: impl Serialize, ctx: &Ctx) {
    let json = serde_json::to_value(&thing).unwrap();
    render_value_as_text("", &json, ctx);
}

fn render_value_as_text(prefix: &str, value: &Value, ctx: &Ctx) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                if is_redacted(key, value, ctx) {
                    continue;
                }
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", prefix, key)
                };
                render_value_as_text(&new_prefix, value, ctx);
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                let new_prefix = if prefix.is_empty() {
                    index.to_string()
                } else {
                    format!("{}[{}]", prefix, index)
                };
                render_value_as_text(&new_prefix, value, ctx);
            }
        }
        _ => {
            println!("{}={}", prefix, value);
        }
    }
}

#[allow(clippy::collapsible_if)]
fn is_redacted(key: &str, _value: &Value, ctx: &Ctx) -> bool {
    if ctx.pretty_redactions {
        if key == "image" {
            return true;
        }
    }
    false
}

fn redact_fields(value: &mut Value, ctx: &Ctx) {
    match value {
        Value::Object(map) => {
            let keys: Vec<String> = map.keys().cloned().collect();
            for key in keys {
                if is_redacted(&key, map.get(&key).unwrap(), ctx) {
                    // map.remove(&key);
                    map.insert(key.clone(), Value::String("...redacted...".to_string()));
                } else {
                    redact_fields(map.get_mut(&key).unwrap(), ctx);
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                redact_fields(item, ctx);
            }
        }
        _ => {}
    }
}
