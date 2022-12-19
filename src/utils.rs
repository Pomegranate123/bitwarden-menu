use crate::serde;
use lazy_static::lazy_static;
use std::env;
use std::process::{Command, Output};

lazy_static! {
    pub static ref CACHE_PATH: String = env::var("XDG_CACHE_HOME").unwrap();
    pub static ref DATA_PATH: String = env::var("XDG_DATA_HOME").unwrap();
}

#[derive(Default)]
pub struct Login {
    pub id: String,
    pub name: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub uri: Option<String>,
    pub notes: Option<String>,
    pub fields: Vec<serde::Field>,
}

impl Login {
    pub fn to_rofi_string(&self) -> String {
        format!(
            "{}\n<i>{}</i>\0icon\x1f{}/bitwarden-menu/images/{}.png\x0f",
            self.name,
            self.username.clone().unwrap_or_default(),
            *CACHE_PATH,
            self.id
        )
    }

    pub fn to_rofi_string_detailed(&self) -> String {
        let mut string = format!(
            "Name:\n<i>{}</i>\0icon\x1f{}/bitwarden-menu/images/{}.png\x0f",
            self.name, *CACHE_PATH, self.id
        );
        if let Some(username) = &self.username {
            string.push_str(&format!("Username:\n<i>{}</i>\x0f", username));
        }
        if let Some(password) = &self.password {
            string.push_str(&format!("Password:\n<i>{}</i>\x0f", password));
        }
        if let Some(uri) = &self.uri {
            string.push_str(&format!("URI:\n<i>{}</i>\x0f", uri));
        }
        if let Some(notes) = &self.notes {
            string.push_str(&format!("Notes:\n<i>{}</i>\x0f", notes));
        }
        for field in &self.fields {
            string.push_str(&format!("{}:\n<i>{}</i>\x0f", field.name, field.value));
        }
        string.replace('&', "&amp;")
    }
}

pub fn run_command(cmd: &str, args: &[&str]) -> Output {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute process: '{}'", cmd));
    let err = parse_output(output.stderr.clone());
    if !err.is_empty() {
        let args: String = args.join(" ");
        eprintln!("Error: {} {}\n{}", cmd, args, err);
    }
    output
}

pub fn parse_output(output: Vec<u8>) -> String {
    String::from_utf8(output)
        .unwrap_or_else(|e| panic!("Invalid UTF-8 sequence: {}", e))
        .trim_end_matches('\n')
        .to_owned()
}

pub trait VisualUnwrap {
    type Output;

    fn unwrap_visual(self) -> Self::Output;
}

impl<T, E: std::fmt::Display> VisualUnwrap for Result<T, E> {
    type Output = T;

    fn unwrap_visual(self) -> Self::Output {
        self.unwrap_or_else(|e| show_error(e))
    }
}

pub fn show_error<E: std::fmt::Display>(err: E) -> ! {
    run_command("rofi", &["-e", &format!("{}", &err)]);
    std::process::exit(1);
}
