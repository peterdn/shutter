use reqwest;
use serde_json;
use serde_json::Value;

#[derive(Deserialize)]
pub struct JsonProfile {
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub biography: Option<String>,
    pub external_url: Option<String>,
    pub profile_pic_url_hd: Option<String>,
    pub edge_owner_to_timeline_media: JsonEdgeOwnerToTimelineMedia,
}

#[derive(Deserialize)]
pub struct JsonEdgeOwnerToTimelineMedia {
    pub edges: Vec<JsonEdge>
}

#[derive(Deserialize)]
pub struct JsonEdge {
    pub node: JsonNode
}

#[derive(Deserialize)]
pub struct JsonNode {
    pub display_url: String
}

fn extract_instagram_json_text(body: &str) -> Result<String, ()> {
    let line = body.lines().filter(|&line| line.contains("window._sharedData =")).nth(0).ok_or(())?;
    let start_idx = line.find('{').ok_or(())?;
    let end_idx = line.rfind('}').ok_or(())? + 1;
    let line = &line[start_idx .. end_idx];
    Ok(line.to_string())
}

fn get_instagram_profile_url(username: &str) -> String {
    format!("https://instagram.com/{}/", username).to_string()
}

fn get_profile_json_value(json_text: &str) -> Result<Value, ()> {
    let json_data: Value = serde_json::from_str(&json_text).unwrap();
    let user_data_json_value = json_data["entry_data"]["ProfilePage"][0]["graphql"]["user"].clone();
    if user_data_json_value.is_null() { Err(()) } else { Ok(user_data_json_value) }
}

pub fn scrape_profile(username: &str) -> Result<JsonProfile, ()> {
    let instagram_profile_url = get_instagram_profile_url(username);
    let response_body: String = reqwest::get(&instagram_profile_url).unwrap().text().unwrap();
    let json_text = extract_instagram_json_text(&response_body)?;
    let user_data_json_value = get_profile_json_value(&json_text)?;
    match serde_json::from_value(user_data_json_value) {
        Ok(profile) => Ok(profile),
        Err(_) => Err(())
    }
}
