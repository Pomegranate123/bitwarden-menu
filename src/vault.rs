use crate::serde;
use crate::session::Session;
use crate::utils::{parse_output, Login};

#[derive(Default)]
pub struct Vault {
    session: Session,
}

impl Vault {
    pub fn get_logins(&mut self) -> Vec<Login> {
        let output = self.session.run_bw(&["list", "items", "--nointeraction"]);
        serde::parse_logins(parse_output(output.stdout))
    }

    pub fn get_login(&mut self, login_id: &str) -> Login {
        let output = self
            .session
            .run_bw(&["get", "item", login_id, "--nointeraction"]);
        serde::parse_login(parse_output(output.stdout))
    }
}
