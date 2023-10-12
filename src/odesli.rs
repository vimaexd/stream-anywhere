use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongLink {
  pub entity_unique_id: String,
  pub url: String,
  pub native_app_uri_mobile: Option<String>,
  pub native_app_uri_desktop: Option<String>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
  pub id: String,
  #[serde(rename = "type")]
  pub entity_type: String,
  pub title: Option<String>,
  pub artist_name: Option<String>,
  pub thumbnail_url: Option<String>,
  pub thumbnail_width: Option<u16>,
  pub thumbnail_height: Option<u16>,
  pub api_provider: String,
  pub platforms: Vec<String> // todo: turn into enum?
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OdesliResponse {
  pub entity_unique_id: String,
  pub user_country: String,
  pub page_url: String,
  pub links_by_platform: HashMap<String, SongLink>,
  pub entities_by_unique_id: HashMap<String, Entity>
}

pub async fn get_song_info(url: &str) -> Result<OdesliResponse, String> {
  let req_url = format!("https://api.song.link/v1-alpha.1/links?songIfSingle=true&url={}", url);
  let _resp = reqwest::get(req_url)
    .await;

  let resp = match _resp {
    Ok(r) => r.json::<OdesliResponse>().await.expect("error serializing response"),
    Err(err) => return Err(format!("error making request: {}", err))
  };

  return Ok(resp);
}