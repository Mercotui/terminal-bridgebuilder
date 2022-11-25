use crate::object::Object;
use anyhow::{anyhow, Context, Result};
use json;
use std::fs;

pub fn load(path: &std::path::PathBuf) -> Result<Vec<Object>> {
    let json_string = fs::read_to_string(path)
        .with_context(|| format!("Could not read file `{}`", path.display()))?;

    let json_data = json::parse(&json_string)
        .with_context(|| format!("Could not parse as json `{}`", path.display()))?;

    parse(json_data)
}

fn parse(root_object: json::JsonValue) -> Result<Vec<Object>> {
    Ok(vec![])
}

pub fn save(objects: &Vec<Object>) -> Result<()> {
    Err(anyhow!("Saving is not implemented yet. {:?}", objects))
}
