use crate::Login;
use serde::Deserialize;

impl From<BwObject> for Login {
    fn from(obj: BwObject) -> Login {
        Login {
            password: obj.password(),
            username: obj.username(),
            uri: obj.uri(),
            name: obj.name,
            id: obj.id,
        }
    }
}

#[derive(Deserialize)]
struct BwObject {
    pub id: String,
    pub name: String,
    pub login: Option<BwLogin>,
    //#[serde(rename = "type")]
    //pub kind: usize,
    //pub notes: Option<String>,
    //#[serde(default)]
    //pub fields: Vec<Field>,
    //pub favorite: bool,
}

#[derive(Default, Deserialize)]
#[serde(rename = "Login")]
struct BwLogin {
    #[serde(default)]
    pub uris: Vec<Uri>,
    pub username: Option<String>,
    pub password: Option<String>,
    //pub totp: Option<String>,
}

#[derive(Deserialize)]
struct Uri {
    pub uri: Option<String>,
    //#[serde(rename = "match")]
    //pub kind: Option<usize>,
}

//#[derive(Deserialize)]
//struct Field {
//    pub name: String,
//    pub value: String,
//}

impl BwObject {
    pub fn username(&self) -> String {
        if let Some(login) = &self.login {
            if let Some(username) = &login.username {
                return username.to_string();
            }
        }
        String::new()
    }

    pub fn password(&self) -> String {
        if let Some(login) = &self.login {
            if let Some(password) = &login.password {
                return password.to_string();
            }
        }
        String::new()
    }

    pub fn uri(&self) -> Option<String> {
        if let Some(login) = &self.login {
            if let Some(uri) = login.uris.first() {
                return uri.uri.clone();
            }
        }
        None
    }
}

pub fn parse_logins(json: String) -> Vec<Login> {
    let objs: Vec<BwObject> = serde_json::from_str(&json).expect("JSON was not well formatted");
    objs.into_iter().map(Login::from).collect()
}

pub fn parse_login(json: String) -> Login {
    let obj: BwObject = serde_json::from_str(&json).expect("JSON was not well formatted");
    obj.into()
}
