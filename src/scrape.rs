use reqwest;
use serde_json;
use serde_json::Value;

#[derive(Debug, Deserialize, PartialEq)]
pub struct JsonProfile {
    pub username: String,
    pub full_name: Option<String>,
    pub biography: Option<String>,
    pub external_url: Option<String>,
    pub profile_pic_url_hd: Option<String>,
    pub is_private: bool,
    pub edge_owner_to_timeline_media: JsonEdgeOwnerToTimelineMedia,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct JsonEdgeOwnerToTimelineMedia {
    pub edges: Vec<JsonEdge>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct JsonEdge {
    pub node: JsonNode,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct JsonNode {
    pub display_url: String,
}

fn extract_instagram_json_text(body: &str) -> Result<String, ()> {
    let line = body
        .lines()
        .filter(|&line| line.contains("window._sharedData ="))
        .nth(0)
        .ok_or(())?;
    let start_idx = line.find('{').ok_or(())?;
    let end_idx = line.rfind('}').ok_or(())? + 1;
    let line = &line[start_idx..end_idx];
    Ok(line.to_string())
}

fn get_instagram_profile_url(username: &str) -> String {
    format!("https://instagram.com/{}/", username).to_string()
}

fn get_profile_json_value(json_text: &str) -> Result<Value, ()> {
    let json_data: Value = serde_json::from_str(&json_text).unwrap();
    let user_data_json_value = json_data["entry_data"]["ProfilePage"][0]["graphql"]["user"].clone();
    if user_data_json_value.is_null() {
        Err(())
    } else {
        Ok(user_data_json_value)
    }
}

fn parse_profile_json(json_text: &str) -> Result<JsonProfile, ()> {
    let user_data_json_value = get_profile_json_value(&json_text)?;
    match serde_json::from_value(user_data_json_value) {
        Ok(profile) => Ok(profile),
        Err(_) => Err(()),
    }
}

pub fn scrape_profile(username: &str) -> Result<JsonProfile, ()> {
    let instagram_profile_url = get_instagram_profile_url(username);
    let response_body: String = reqwest::get(&instagram_profile_url)
        .unwrap()
        .text()
        .unwrap();
    let json_text = extract_instagram_json_text(&response_body)?;
    parse_profile_json(&json_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_instagram_json_text() {
        {
            let nominal_body = r#"<test>
                    window._sharedData = {"username": "peterdn"}
                    </test>"#;
            let nominal_json_text = extract_instagram_json_text(&nominal_body);
            assert_eq!(
                nominal_json_text,
                Ok(r#"{"username": "peterdn"}"#.to_string())
            );
        }

        {
            let nominal_body = r#"<test>
                    window._sharedData = {"username": "peterdn", "data": {}}
                    </test>"#;
            let nominal_json_text = extract_instagram_json_text(&nominal_body);
            assert_eq!(
                nominal_json_text,
                Ok(r#"{"username": "peterdn", "data": {}}"#.to_string())
            );
        }

        {
            let invalid_body = r#"<test>
                    window._sharedData = notrealjson
                    </test>"#;
            let invalid_json_text = extract_instagram_json_text(&invalid_body);
            assert_eq!(invalid_json_text, Err(()));
        }

        {
            let invalid_body = r#"<test>
                    x = y
                    </test>"#;
            let invalid_json_text = extract_instagram_json_text(&invalid_body);
            assert_eq!(invalid_json_text, Err(()));
        }

        {
            let invalid_body = r#"<test>
                    window._badData = {"username": "peterdn"}
                    </test>"#;
            let invalid_json_text = extract_instagram_json_text(&invalid_body);
            assert_eq!(invalid_json_text, Err(()));
        }
    }

    #[test]
    fn test_get_profile_json_data() {
        {
            let nominal_json = r#"{
                "entry_data": {
                    "ProfilePage": [{
                        "graphql": {
                            "user": {
                                "username": "peterdn",
                                "full_name": "Peter Nelson",
                                "biography": "test biography",
                                "external_url": "https://peterdn.com",
                                "profile_pic_url_hd": "https://peterdn.com/profile.jpg",
                                "is_private": false,
                                "edge_owner_to_timeline_media": {
                                    "edges": [{
                                        "node": {"display_url": "https://peterdn.com/1.jpg"}
                                    },{
                                        "node": {"display_url": "https://peterdn.com/2.jpg"}
                                    }]
                                }
                            }
                        }
                    }]
                }
            }"#;
            let nominal_profile_value = parse_profile_json(&nominal_json.to_string());
            assert_eq!(
                nominal_profile_value,
                Ok(JsonProfile {
                    username: "peterdn".to_string(),
                    full_name: Some("Peter Nelson".to_string()),
                    biography: Some("test biography".to_string()),
                    external_url: Some("https://peterdn.com".to_string()),
                    profile_pic_url_hd: Some("https://peterdn.com/profile.jpg".to_string()),
                    is_private: false,
                    edge_owner_to_timeline_media: JsonEdgeOwnerToTimelineMedia {
                        edges: vec![
                            JsonEdge {
                                node: JsonNode {
                                    display_url: "https://peterdn.com/1.jpg".to_string(),
                                },
                            },
                            JsonEdge {
                                node: JsonNode {
                                    display_url: "https://peterdn.com/2.jpg".to_string(),
                                },
                            },
                        ],
                    },
                })
            );
        }

        {
            let empty_json = r#"{
                "entry_data": {
                    "ProfilePage": [{
                        "graphql": {
                            "user": {
                                "username": "testuser",
                                "full_name": null,
                                "biography": null,
                                "external_url": null,
                                "profile_pic_url_hd": null,
                                "is_private": false,
                                "edge_owner_to_timeline_media": {"edges": []}
                            }
                        }
                    }]
                }
            }"#;
            let empty_profile_value = parse_profile_json(&empty_json.to_string());
            assert_eq!(
                empty_profile_value,
                Ok(JsonProfile {
                    username: "testuser".to_string(),
                    full_name: None,
                    biography: None,
                    external_url: None,
                    profile_pic_url_hd: None,
                    is_private: false,
                    edge_owner_to_timeline_media: JsonEdgeOwnerToTimelineMedia { edges: vec![] },
                })
            );
        }

        {
            let incomplete_json = r#"{
                "entry_data": {
                    "ProfilePage": [{ "graphql": { "user": {
                        "username": "testuser", "biography": null, "external_url": null, "is_private": false,
                        "profile_pic_url_hd": null, "edge_owner_to_timeline_media": {"edges": []}
                    }}}]
                }
            }"#;
            let incomplete_profile_value = parse_profile_json(&incomplete_json.to_string());
            assert_eq!(
                incomplete_profile_value,
                Ok(JsonProfile {
                    username: "testuser".to_string(),
                    full_name: None,
                    biography: None,
                    external_url: None,
                    profile_pic_url_hd: None,
                    is_private: false,
                    edge_owner_to_timeline_media: JsonEdgeOwnerToTimelineMedia { edges: vec![] },
                })
            );
        }

        {
            let incomplete_json = r#"{
                "entry_data": {
                    "ProfilePage": [{ "graphql": { "user": {
                        "username": null, "full_name": null, "biography": null,
                        "external_url": null, "profile_pic_url_hd": null
                    }}}]
                }
            }"#;
            let incomplete_profile_value = parse_profile_json(&incomplete_json.to_string());
            assert_eq!(incomplete_profile_value, Err(()));
        }
    }
}
