#![feature(result_option_inspect)]

use crate::utils::{Login, Session};
use copypasta_ext::prelude::*;
use copypasta_ext::x11_bin::ClipboardContext;
use std::env;
use std::path::Path;

pub mod cache;
pub mod serde;
pub mod utils;

fn main() {
    let cache_path = env::var("XDG_CACHE_HOME").unwrap();
    let mut session = Session::new();
    let mut ctx = ClipboardContext::new().unwrap();

    if !Path::new(&format!("{}/bw-rofi/rofi.txt", cache_path)).exists()
        || !Path::new(&format!("{}/bw-rofi/cache.txt", cache_path)).exists()
    {
        cache::write(&session.get_logins());
    }

    let ids = cache::read_ids().unwrap();

    let index = utils::run_command(
        "rofi",
        &[
            "-dmenu",
            "-sep",
            "\x0f",
            "-p",
            "Bitwarden:",
            "-format",
            "i",
            "-markup-rows",
            "-input",
            &format!("{}/bw-rofi/rofi.txt", cache_path),
        ],
    );

    let id = ids.get(index.parse::<usize>().unwrap()).unwrap();
    let login = session.get_login(id);
    ctx.set_contents(login.password).unwrap();
}
