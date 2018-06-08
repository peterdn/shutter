#[macro_use]
extern crate clap;
extern crate rayon;
extern crate reqwest;
extern crate shutter;

use clap::App;
use rayon::prelude::*;
use std::io::BufWriter;
use std::fs::File;
use std::process;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

fn print_profile(user_profile: &shutter::profile::Profile) {
    println!("Username: {}", user_profile.username.clone().unwrap_or("".to_string()));
    println!("Full name: {}", user_profile.full_name.clone().unwrap_or("".to_string()));
    println!("Biography:\n{}", user_profile.biography.clone().unwrap_or("".to_string()));
    println!("URL: {}", user_profile.external_url.clone().unwrap_or("".to_string()));
    println!("Private profile: {}", user_profile.is_private.unwrap_or(false));
    println!("Profile picture: {}", user_profile.profile_pic.clone().unwrap().url);
}

fn download_images(user_profile: &shutter::profile::Profile) {
    println!("Downloading {} images...", user_profile.images.len());
    user_profile.images.par_iter().for_each(|ref img| {
        let filename = img.url.rsplit('/').collect::<Vec<&str>>()[0];
        println!("Downloading {}...", filename);
        let image_file = File::create(filename).unwrap();
        let mut writer = BufWriter::new(image_file);
        reqwest::get(&img.url).unwrap().copy_to(&mut writer);
    });
}

fn main() {
    let args_yaml_schema = load_yaml!("args.yaml");
    let args = App::from_yaml(args_yaml_schema).get_matches();

    let username = args.value_of("USERNAME").unwrap();

    if let Ok(user_profile) = shutter::profile::Profile::get(username) {
        if args.is_present("profile") {
            print_profile(&user_profile);
        }

        if args.is_present("images") {
            download_images(&user_profile);
        }

        process::exit(EXIT_SUCCESS);

    } else {
        println!("User {} could not be found", username);
        process::exit(EXIT_FAILURE);
    }
}
