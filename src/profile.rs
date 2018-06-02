use image::Image;
use scrape;

pub struct Profile {
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub biography: Option<String>,
    pub external_url: Option<String>,
    pub profile_pic: Option<Image>,
    pub images: Vec<Image>,
}

impl Profile {
    pub fn get(username: &str) -> Result<Profile, ()> {
        let json_profile = scrape::scrape_profile(username)?;

        let profile_pic_image = json_profile.profile_pic_url_hd.map(|url| Image { url });
        let images = json_profile.edge_owner_to_timeline_media.edges.iter().map(|ref edge| {
            Image { url: edge.node.display_url.clone() }
        }).collect::<Vec<Image>>();

        Ok(Profile {
            username: json_profile.username,
            full_name: json_profile.full_name,
            biography: json_profile.biography,
            external_url: json_profile.external_url,
            profile_pic: profile_pic_image,
            images
        })
    }
}
