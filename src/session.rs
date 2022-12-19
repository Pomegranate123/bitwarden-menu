use crate::utils::{parse_output, run_command};
use std::process::{Command, Output};

#[derive(Default)]
pub struct Session(Option<String>);

impl Session {
    pub fn run_bw(&mut self, args: &[&str]) -> Output {
        let session_args = [args, &["--session", self.get()]].concat();
        let output = Command::new("bw")
            .args(&session_args)
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute process: 'bw'"));
        let err = parse_output(output.stderr.clone());
        match err.as_str() {
            "mac failed." => {
                self.refresh(false);
                self.run_bw(args);
            }
            "" => (),
            _ => {
                panic!("Error: bw {}\n{}", session_args.join(" "), err);
            }
        }
        output
    }

    fn get(&mut self) -> &str {
        if self.0.is_none() {
            self.refresh(true);
        }
        self.0.as_ref().unwrap()
    }

    fn refresh(&mut self, use_cache: bool) {
        let session_id = if use_cache {
            match Session::get_cached() {
                Some(session_id) => session_id,
                None => Session::get_new(),
            }
        } else {
            Session::get_new()
        };

        let keyctl = run_command("keyctl", &["add", "user", "bw_session", &session_id, "@u"]);
        // Set key timeout to 15 minutes (900 seconds)
        run_command("keyctl", &["timeout", &parse_output(keyctl.stdout), "900"]);
        self.0 = Some(session_id);
    }

    fn get_cached() -> Option<String> {
        let keyctl = run_command("keyctl", &["request", "user", "bw_session"]);
        if keyctl.status.success() {
            let keyctl = run_command("keyctl", &["pipe", &parse_output(keyctl.stdout)]);
            if keyctl.status.success() {
                return Some(parse_output(keyctl.stdout));
            }
        }
        None
    }

    fn get_new() -> String {
        let mut password = Session::get_password(None);
        loop {
            let unlock = run_command("bw", &["--raw", "unlock", &password, "--nointeraction"]);
            if unlock.status.success() {
                return parse_output(unlock.stdout);
            }
            password = Session::get_password(Some(&parse_output(unlock.stderr)));
        }
    }

    fn get_password(err: Option<&str>) -> String {
        let mut args = vec!["-dmenu", "-l", "0", "-password", "-p", "Password:"];
        if let Some(e) = err {
            args.extend(["-mesg", e]);
        }
        let output = run_command("rofi", &args);
        if let Some(1) = output.status.code() {
            std::process::exit(0);
        }
        parse_output(output.stdout)
    }
}
