use serde_json::Value;

/// Gives the type Name of a Value, like "Null", or "String", or "Bool".
pub fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "Null",
        Value::Bool(_) => "Bool",
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
    }
}

/// Applies a series of patches to a json value.
/// patches is read in chunks of 2, first being the json pointer, and second being
/// a string that will be parsed as json, and must be the same type as the value at the pointer.
/// value at the pointer will be mutated in place, with the new value from the patch.
/// any pointers that don't exist or mismatched replacement types will cause a panic.
pub fn apply_patches(json: &mut Value, patches: &[String]) -> Result<(), anyhow::Error> {
    for chunks in patches.chunks(2) {
        let pointer = &chunks[0];
        let new_val = &chunks[1];
        if let Some(value) = json.pointer_mut(pointer) {
            // parse new value as json
            let Ok(new_value) = serde_json::from_str::<Value>(new_val) else {
                return Err(anyhow::anyhow!(
                    "Unable to parse {new_val} as a valid json value"
                ));
            };
            let existing_type = value_type_name(value);
            let new_type = value_type_name(&new_value);
            // Check that new_value type matches the existing value type for this field
            if existing_type != new_type {
                return Err(anyhow::anyhow!(
                    "Type mismatch: Cannot convert {existing_type} to {new_type}\nExisting value: {}\nPatched value: {}",
                    value,
                    new_value
                ));
            }
            // replace the field
            // println!("Replacing {value} with {new_value} @ {pointer}");
            *value = new_value;
        } else {
            return Err(anyhow::anyhow!("Pointer {pointer} not found in json"));
        }
    }
    Ok(())
}
