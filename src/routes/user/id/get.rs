use axum::{extract::Path, Json, http::StatusCode};
use serde_json::{Value, json};

use crate::lib::cache::{MEMBERS, VOICE_CHANNELS};

pub async fn trigger(Path(user_id): Path<u64>) -> Result<Json<Value>, StatusCode> {

  let members = MEMBERS.lock().await;
  let member = if let Some(member) = members.get(&user_id) {
    let member = member.clone();
    drop(members);
    if member.is_alive().map_err(|_| StatusCode::NOT_EXTENDED)? {
      member
    } else {
      return Err(StatusCode::NOT_FOUND)
    }
  } else {
    return Err(StatusCode::NOT_FOUND)
  };

  let voices = VOICE_CHANNELS.lock().await;
  let voice = if let Some(voice) = VOICE_CHANNELS.lock().await.get(&member.voice_channel_id) {
    let voice = voice.clone();
    drop(voices);
    voice
  } else {
    return Err(StatusCode::NOT_FOUND)
  };

  return Ok(Json(json!({
    "ok": true,
    "data": {
      "member": member,
      "voice": voice
    }
  })))
}