#[macro_use]
extern crate clap;
extern crate rayon;
extern crate reqwest;
extern crate shutter;

use clap::App;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process;
use std::time::UNIX_EPOCH;

use shutter::error::Result;
use shutter::image::PostImage;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

fn print_profile(user_profile: &shutter::profile::Profile) {
    println!("Username: {}", user_profile.username);
    println!(
        "Full name: {}",
        user_profile.full_name.as_ref().unwrap_or(&"".to_string())
    );
    println!(
        "Biography:\n{}",
        user_profile.biography.as_ref().unwrap_or(&"".to_string())
    );
    println!(
        "URL: {}",
        user_profile
            .external_url
            .as_ref()
            .unwrap_or(&"".to_string())
    );
    println!("Private profile: {}", user_profile.is_private);
    println!(
        "Profile picture: {}",
        user_profile.profile_pic.as_ref().unwrap().url
    );
}

fn get_local_filepath_for_image(outdir: &Path, img: &PostImage) -> PathBuf {
    let timestamp_s = img
        .timestamp_uploaded
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("{}.jpg", timestamp_s);
    outdir.join(filename)
}

fn download_images(user_profile: &shutter::profile::Profile, outdir: &Path) {
    if outdir.exists() && !outdir.is_dir() {
        panic!("A file exists with the same name as the output directory!");
    }

    std::fs::create_dir(&outdir).expect("Failed to create output directory!");

    println!("Downloading {} images...", user_profile.images.len());
    user_profile.images.par_iter().for_each(|ref img| {
        let filepath = get_local_filepath_for_image(&outdir, &img);
        println!("Downloading {}", filepath.to_string_lossy());
        let image_file = File::create(filepath).unwrap();
        let mut writer = BufWriter::new(image_file);
        reqwest::get(&img.url)
            .unwrap()
            .copy_to(&mut writer)
            .expect("Failed to download to file");
    });
}

fn do_main() -> Result<()> {
    let args_yaml_schema = load_yaml!("args.yaml");
    let args = App::from_yaml(args_yaml_schema).get_matches();

    let username = args.value_of("USERNAME").unwrap();

    match shutter::profile::Profile::get(username) {
        Ok(user_profile) => {
            if args.is_present("profile") {
                print_profile(&user_profile);
            }

            if args.is_present("images") {
                let default_outdir = format!("{}_images", username);
                let outdir = Path::new(args.value_of("outdir").unwrap_or(&default_outdir));
                download_images(&user_profile, &outdir);
            }

            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}

fn main() -> ! {
    match do_main() {
        Ok(()) => process::exit(EXIT_SUCCESS),
        _ => process::exit(EXIT_FAILURE),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    #[test]
    fn test_get_local_filepath_for_image() {
        let timestamp = UNIX_EPOCH + Duration::from_secs(1230000000);
        let image = PostImage {
            url: "doesntmatter.jpg".to_string(),
            timestamp_uploaded: timestamp,
        };
        let outdir = Path::new("/test/dir");
        assert_eq!(
            get_local_filepath_for_image(&outdir, &image),
            Path::new("/test/dir/1230000000.jpg")
        );
    }
}
