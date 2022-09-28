use crate::Login;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn read_ids() -> Option<Vec<String>> {
    let cache_path = env::var("XDG_CACHE_HOME").unwrap();
    if Path::new(&format!("{}/bitwarden-menu/cache.txt", cache_path)).exists() {
        let cache: Vec<String> = serde_json::from_str(
            &fs::read_to_string(format!("{}/bitwarden-menu/cache.txt", cache_path))
                .expect("Unable to read file"),
        )
        .unwrap();
        Some(cache)
    } else {
        None
    }
}

pub fn write(logins: &[Login]) {
    let cache_path = std::env::var("XDG_CACHE_HOME").unwrap();
    fs::create_dir_all(format!("{}/bitwarden-menu/images", cache_path)).unwrap();

    let mut rofi_cache = fs::File::create(format!("{}/bitwarden-menu/rofi.txt", cache_path)).unwrap();
    for login in logins {
        cache_image(login);
        write!(
            rofi_cache,
            "{}",
            format!(
                "{}\n<i>{}</i>\0icon\x1f{}/bitwarden-menu/images/{}.png\x0f",
                login.name, login.username, cache_path, login.id
            )
            .replace('&', "&amp;")
        )
        .unwrap();
    }

    let mut buffer = fs::File::create(format!("{}/bitwarden-menu/cache.txt", cache_path)).unwrap();
    let ids: Vec<String> = logins.iter().map(|login| login.id.clone()).collect();
    let serialized = serde_json::to_string(&ids).unwrap();
    write!(buffer, "{}", serialized).unwrap();
}

fn cache_image(login: &Login) {
    let cache_path = env::var("XDG_CACHE_HOME").unwrap();
    let data_path = env::var("XDG_DATA_HOME").unwrap();
    if Path::new(&format!("{}/bitwarden-menu/images/{}.png", cache_path, login.id)).exists() {
        return;
    }
    match &login.uri {
        Some(website) => {
            let website = website.trim_start_matches("https://");
            let mut file =
                fs::File::create(format!("{}/bitwarden-menu/images/{}.png", cache_path, login.id))
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
            format!("{}/bitwarden-menu/default.png", data_path),
            format!("{}/bitwarden-menu/images/{}.png", cache_path, login.id),
        )
        .unwrap(),
    }
}
