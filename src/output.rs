use anyhow::Result;
use serde::Serialize;

pub struct Output {
    // pub is_json: bool,
}

impl Output {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print<T: Serialize>(&self, data: &T) -> Result<()> {
        println!("{}", serde_json::to_string_pretty(data)?);
        Ok(())
    }
}
