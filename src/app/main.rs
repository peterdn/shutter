extern crate shutter;

fn main() {
    if let Ok(user_profile) = shutter::profile::Profile::get("peterdn") {
        println!("{:?}", user_profile.username);
        println!("{:?}", user_profile.full_name);
        println!("{:?}", user_profile.biography);
        println!("{:?}", user_profile.external_url);
        println!("{}", user_profile.profile_pic.unwrap().url);

        for ref img in user_profile.images {
            println!("{}", img.url);
        }
    }
}
