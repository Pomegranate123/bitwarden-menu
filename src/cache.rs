use crate::utils;
use crate::Login;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn read_ids() -> Option<Vec<String>> {
    if Path::new(&format!("{}/bitwarden-menu/cache", *utils::CACHE_PATH)).exists() {
        let cache: Vec<String> = serde_json::from_str(
            &fs::read_to_string(format!("{}/bitwarden-menu/cache", *utils::CACHE_PATH))
                .expect("Unable to read file"),
        )
        .unwrap();
        Some(cache)
    } else {
        None
    }
}

pub fn write(logins: &[Login]) {
    fs::create_dir_all(format!("{}/bitwarden-menu/images", *utils::CACHE_PATH)).unwrap();

    let mut rofi_cache =
        fs::File::create(format!("{}/bitwarden-menu/rofi", *utils::CACHE_PATH)).unwrap();
    for login in logins {
        cache_image(login);
        write!(rofi_cache, "{}", login.to_rofi_string()).unwrap();
    }

    let mut buffer =
        fs::File::create(format!("{}/bitwarden-menu/cache", *utils::CACHE_PATH)).unwrap();
    let ids: Vec<String> = logins.iter().map(|login| login.id.clone()).collect();
    let serialized = serde_json::to_string(&ids).unwrap();
    write!(buffer, "{}", serialized).unwrap();
}

fn cache_image(login: &Login) {
    if Path::new(&format!(
        "{}/bitwarden-menu/images/{}.png",
        *utils::CACHE_PATH,
        login.id
    ))
    .exists()
    {
        return;
    }
    match &login.uri {
        Some(website) => {
            let website = website
                .trim_start_matches("https://")
                .trim_start()
                .trim_end();
            println!("{}", website);
            let mut file = fs::File::create(format!(
                "{}/bitwarden-menu/images/{}.png",
                *utils::CACHE_PATH,
                login.id
            ))
            .unwrap();
            let request_url = format!(
                "https://s2.googleusercontent.com/s2/favicons?domain_url=https://{}&sz=96",
                &website
            );
            reqwest::blocking::get(request_url)
                .unwrap()
                .copy_to(&mut file)
                .unwrap();
        }
        None => std::os::unix::fs::symlink(
            format!("{}/bitwarden-menu/default.png", *utils::DATA_PATH),
            format!(
                "{}/bitwarden-menu/images/{}.png",
                *utils::CACHE_PATH,
                login.id
            ),
        )
        .unwrap(),
    }
}
