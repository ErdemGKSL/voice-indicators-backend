use anyhow::Result;

use serde::{Serialize, Deserialize};

use super::get_current_milis;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct VoiceChannel {
  pub id: u64,
  pub channel_name: String,
  pub is_dm: bool
}

#[derive(Clone, Serialize, Debug)]
pub struct Member {
  pub id: u64,
  pub voice_channel_id: u64,
  pub until: u128
}

impl Member {
  pub fn is_alive(&self) -> Result<bool> {
    let now = get_current_milis()?;
    Ok(self.until > now)
  }
}

