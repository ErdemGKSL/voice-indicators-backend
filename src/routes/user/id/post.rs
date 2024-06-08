use axum::{extract::{Path, Json}, http::StatusCode};
use serde_json::{Value, json};

use crate::lib::{structs::{VoiceChannel, Member}, cache::{MEMBERS, VOICE_CHANNELS}, get_current_milis};

pub async fn trigger(Path(user_id): Path<u64>, Json(body): Json<VoiceChannel>) -> Result<Json<Value>, StatusCode> {

  let member = Member {
    id: user_id,
    voice_channel_id: body.id,
    until: get_current_milis().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? + (1000 * 60 * 3),
  };

  {
    let mut members = MEMBERS.lock().await;
    members.remove(&user_id);
    members.insert(user_id, member);
    drop(members);
  }

  {
    let mut voices = VOICE_CHANNELS.lock().await;
    voices.remove(&body.id);
    voices.insert(body.id, body);
    drop(voices);
  }

  Ok(Json(json!({
    "ok": true
  })))
}