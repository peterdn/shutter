use error::Result;
use image::{PostImage, ProfileImage};
use scrape;

#[derive(Debug, PartialEq)]
pub struct Profile {
    pub username: String,
    pub full_name: Option<String>,
    pub biography: Option<String>,
    pub external_url: Option<String>,
    pub profile_pic: Option<ProfileImage>,
    pub is_private: bool,
    pub images: Vec<PostImage>,
}

impl From<scrape::JsonProfile> for Profile {
    fn from(json_profile: scrape::JsonProfile) -> Profile {
        let profile_pic_image = json_profile
            .profile_pic_url_hd
            .map(|url| ProfileImage { url });
        let images = json_profile
            .edge_owner_to_timeline_media
            .edges
            .iter()
            .map(|ref edge| PostImage::new(&edge.node.display_url, edge.node.taken_at_timestamp))
            .collect::<Vec<PostImage>>();

        Profile {
            username: json_profile.username,
            full_name: json_profile.full_name,
            biography: json_profile.biography,
            external_url: json_profile.external_url,
            profile_pic: profile_pic_image,
            is_private: json_profile.is_private,
            images,
        }
    }
}

impl Profile {
    pub fn get(username: &str) -> Result<Profile> {
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
            username: "peterdn".to_string(),
            full_name: Some("Peter Nelson".to_string()),
            biography: Some("test biography".to_string()),
            external_url: Some("https://peterdn.com".to_string()),
            profile_pic_url_hd: Some("https://peterdn.com/profile.jpg".to_string()),
            is_private: true,
            edge_owner_to_timeline_media: JsonEdgeOwnerToTimelineMedia {
                edges: vec![
                    JsonEdge {
                        node: JsonNode {
                            display_url: "https://peterdn.com/1.jpg".to_string(),
                            taken_at_timestamp: 1200000000,
                        },
                    },
                    JsonEdge {
                        node: JsonNode {
                            display_url: "https://peterdn.com/2.jpg".to_string(),
                            taken_at_timestamp: 1300000000,
                        },
                    },
                ],
            },
        };
        assert_eq!(
            Profile::from(json_profile),
            Profile {
                username: "peterdn".to_string(),
                full_name: Some("Peter Nelson".to_string()),
                biography: Some("test biography".to_string()),
                external_url: Some("https://peterdn.com".to_string()),
                profile_pic: Some(ProfileImage {
                    url: "https://peterdn.com/profile.jpg".to_string()
                }),
                is_private: true,
                images: vec![
                    PostImage::new("https://peterdn.com/1.jpg", 1200000000),
                    PostImage::new("https://peterdn.com/2.jpg", 1300000000),
                ],
            }
        );
    }

    #[test]
    fn test_get_instagram_profile() {
        // Use `instagram` user as this one is likely to always exist...
        let profile = Profile::get("instagram");
        assert!(profile.is_ok());

        let profile = profile.unwrap();
        assert_eq!(profile.username, "instagram".to_string());
        assert!(!profile.is_private);
        assert_eq!(profile.images.len(), 12);
    }
}
