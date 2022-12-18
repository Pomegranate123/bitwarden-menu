#![feature(result_option_inspect)]

use crate::utils::{Login, Session};
use copypasta_ext::prelude::*;
use copypasta_ext::x11_bin::ClipboardContext;
use notify_rust::Notification;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub mod cache;
pub mod serde;
pub mod utils;

fn main() {
    let mut session = Session::new();
    let mut ctx = ClipboardContext::new().unwrap();

    if !Path::new(&format!("{}/bitwarden-menu/rofi", *utils::CACHE_PATH)).exists()
        || !Path::new(&format!("{}/bitwarden-menu/cache", *utils::CACHE_PATH)).exists()
        || !Path::new(&format!("{}/bitwarden-menu/images/", *utils::CACHE_PATH)).exists()
    {
        cache::write(&session.get_logins());
    }

    let ids = cache::read_ids().unwrap();
    let mut index: usize = 0;

    loop {
        let output = Command::new("rofi")
            .args(&[
                "-dmenu",       // Launch in dmenu mode
                "-sep",         // Set the seperator between entries to
                "\x0f",         // \x0f to allow for \n
                "-p",           // Set the prompt text to
                "Bitwarden:",   // Bitwarden:
                "-i",           // Case-insensitive
                "-no-custom",   // Don't allow custom input
                "-format",      // Set return type to
                "i",            // index of entry instead of name
                "-markup-rows", // Parse pango markup
                "-selected-row",
                &index.to_string(),
                "-mesg",
                "Alt+1 - Show details",
                "-input", // Select input file
                &format!("{}/bitwarden-menu/rofi", *utils::CACHE_PATH),
            ])
            .output()
            .expect("Failed to execute process");
        let stdout = utils::parse_output(output.stdout);
        match output.status.code().unwrap() {
            1 => {
                println!("Return");
                return;
            }
            0 => {
                println!("Copy password");
                // Copy password
                index = stdout.parse::<usize>().unwrap();
                let id = ids.get(index).unwrap();
                let login = session.get_login(id);
                match login.password {
                    Some(password) => {
                        ctx.set_contents(password).unwrap();
                        Notification::new()
                            .summary(&login.name)
                            .body("Password copied to clipboard")
                            .icon(&format!(
                                "file://{}/bitwarden-menu/images/{}.png",
                                *utils::CACHE_PATH,
                                id
                            ))
                            .show()
                            .unwrap();
                        return;
                    }
                    None => utils::show_error(&format!(
                        "Login entry '{}' does not have a password.",
                        login.name
                    )),
                }
            }
            10 => {
                println!("Show login");
                // Show login
                index = stdout.parse::<usize>().unwrap();
                let id = ids.get(index).unwrap();
                let login = session.get_login(id);
                let mut rofi = Command::new("rofi")
                    .args(&[
                        "-dmenu",       // Launch in dmenu mode
                        "-sep",         // Set the seperator between entries to
                        "\x0f",         // \x0f to allow for \n
                        "-i",           // Case-insensitive
                        "-no-custom",   // Don't allow custom input
                        "-format",      // Set return type to
                        "i",            // index of entry instead of name
                        "-markup-rows", // Parse pango markup
                    ])
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("Failed to execute process");

                let mut stdin = rofi.stdin.take().expect("Failed to open stdin");
                std::thread::spawn(move || {
                    stdin
                        .write_all(login.to_rofi_string_detailed().as_bytes())
                        .expect("Failed to write to stdin");
                });

                let output = rofi.wait_with_output().expect("Failed to read stdout");
                println!("{}", utils::parse_output(output.stdout));
            }
            _ => println!("index: {}, code: {}", index, output.status.code().unwrap()),
        }
    }
}
