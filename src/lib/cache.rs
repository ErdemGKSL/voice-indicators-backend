use std::{sync::Arc, collections::HashMap};
use lazy_static::lazy_static;
use tokio::sync::Mutex;

use super::structs::{VoiceChannel, Member};

lazy_static! {
  pub static ref VOICE_CHANNELS: Arc<Mutex<HashMap<u64, VoiceChannel>>> = Arc::new(Mutex::new(HashMap::new()));
  pub static ref MEMBERS: Arc<Mutex<HashMap<u64, Member>>> = Arc::new(Mutex::new(HashMap::new()));
}