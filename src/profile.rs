use image::Image;
use scrape;

#[derive(Debug, PartialEq)]
pub struct Profile {
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub biography: Option<String>,
    pub external_url: Option<String>,
    pub profile_pic: Option<Image>,
    pub is_private: Option<bool>,
    pub images: Vec<Image>,
}

impl From<scrape::JsonProfile> for Profile {
    fn from(json_profile: scrape::JsonProfile) -> Profile {
        let profile_pic_image = json_profile.profile_pic_url_hd.map(|url| Image { url });
        let images = json_profile.edge_owner_to_timeline_media.edges.iter().map(|ref edge| {
            Image { url: edge.node.display_url.clone() }
        }).collect::<Vec<Image>>();

        Profile {
            username: json_profile.username,
            full_name: json_profile.full_name,
            biography: json_profile.biography,
            external_url: json_profile.external_url,
            profile_pic: profile_pic_image,
            is_private: json_profile.is_private,
            images
        }
    }
}

impl Profile {
    pub fn get(username: &str) -> Result<Profile, ()> {
        let json_profile = scrape::scrape_profile(username)?;
        Ok(Profile::from(json_profile))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::*;
    use scrape::*;

    #[test]
    fn test_from_json_profile() {
        let json_profile = JsonProfile {
            username: Some("peterdn".to_string()),
            full_name: Some("Peter Nelson".to_string()),
            biography: Some("test biography".to_string()),
            external_url: Some("https://peterdn.com".to_string()),
            profile_pic_url_hd: Some("https://peterdn.com/profile.jpg".to_string()),
            is_private: Some(true),
            edge_owner_to_timeline_media: JsonEdgeOwnerToTimelineMedia {
                edges: vec![JsonEdge {
                    node: JsonNode {display_url: "https://peterdn.com/1.jpg".to_string()}
                }, JsonEdge {
                    node: JsonNode {display_url: "https://peterdn.com/2.jpg".to_string()}
                }]
            }
        };
        assert_eq!(Profile::from(json_profile), Profile {
            username: Some("peterdn".to_string()),
            full_name: Some("Peter Nelson".to_string()),
            biography: Some("test biography".to_string()),
            external_url: Some("https://peterdn.com".to_string()),
            profile_pic: Some(Image { url: "https://peterdn.com/profile.jpg".to_string() }),
            is_private: Some(true),
            images: vec![
                Image {url: "https://peterdn.com/1.jpg".to_string()},
                Image {url: "https://peterdn.com/2.jpg".to_string()},
            ]
        });
    }
}
