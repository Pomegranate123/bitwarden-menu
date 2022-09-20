use crate::serde;
use std::process::Command;

pub struct Login {
    pub id: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub uri: Option<String>,
}

#[derive(Default)]
pub struct Session(Option<String>);

impl Session {
    pub fn new() -> Self {
        Session(None)
    }

    pub fn get_logins(&mut self) -> Vec<Login> {
        let json = run_command("bw", &["list", "items", "--session", &self.id()]);
        serde::parse_logins(json)
    }

    pub fn get_login(&mut self, login_id: &str) -> Login {
        let json = run_command("bw", &["get", "item", login_id, "--session", &self.id()]);
        serde::parse_login(json)
    }

    fn id(&mut self) -> String {
        match &self.0 {
            Some(id) => id.to_owned(),
            None => {
                let mut session_id: Option<String> = None;
                let mut err: Option<String> = None;
                while session_id.is_none() {
                    let password = password_prompt(&err);
                    if password.is_empty() {
                        std::process::exit(0);
                    }
                    session_id = try_run_command("bw", &["--raw", "unlock", &password])
                        .inspect_err(|e| err = Some(e.to_owned()))
                        .ok();
                }
                self.0 = session_id.clone();
                session_id.unwrap()
            }
        }
    }
}

fn password_prompt(err: &Option<String>) -> String {
    let mut args = vec!["-dmenu", "-l", "0", "-password", "-p", "Password:"];
    if let Some(e) = err {
        args.extend(&["-mesg", e]);
    }
    run_command("rofi", &args)
}

pub fn show_error(err: &str) -> ! {
    try_run_command("rofi", &["-e", err]).unwrap();
    std::process::exit(1);
}

pub fn try_run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute process");
    let stderr = parse_output(output.stderr);
    if !stderr.is_empty() {
        Err(stderr)
    } else {
        Ok(parse_output(output.stdout))
    }
}

pub fn run_command(cmd: &str, args: &[&str]) -> String {
    try_run_command(cmd, args).unwrap_or_else(|e| show_error(&e))
}

fn parse_output(output: Vec<u8>) -> String {
    String::from_utf8(output)
        .unwrap_or_else(|e| panic!("Invalid UTF-8 sequence: {}", e))
        .trim_end_matches('\n')
        .to_owned()
}
