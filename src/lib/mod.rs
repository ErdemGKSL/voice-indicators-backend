use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;

pub mod cache;
pub mod structs;

pub fn get_current_milis() -> Result<u128> {
  let time = SystemTime::now().duration_since(UNIX_EPOCH)?;

  return Ok(time.as_millis());
}