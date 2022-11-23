use crate::object::Object;
use anyhow::{anyhow, Context, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn load(path: &std::path::PathBuf, pattern: &String) -> Result<Vec<Object>> {
    let file =
        File::open(path).with_context(|| format!("could not read file `{}`", path.display()))?;
    let file_reader = BufReader::new(file);

    for line_result in file_reader.lines() {
        let line = line_result?;
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    Ok(vec![])
}

pub fn save(objects: &Vec<Object>) -> Result<()> {
    Err(anyhow!("Saving is not implemented yet. {:?}", objects))
}
