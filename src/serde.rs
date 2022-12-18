use crate::Login;
use serde::Deserialize;

impl From<BwObject> for Login {
    fn from(obj: BwObject) -> Login {
        Login {
            id: obj.id,
            name: obj.name,
            notes: obj.notes,
            username: obj.login.as_ref().and_then(|login| login.username.clone()),
            password: obj.login.as_ref().and_then(|login| login.password.clone()),
            fields: obj.fields,
            uri: obj.login.and_then(|l| {
                l.uris
                    .first()
                    .map(|uri| uri.uri.clone().unwrap_or_default())
            }),
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
    pub notes: Option<String>,
    #[serde(default)]
    pub fields: Vec<Field>,
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

#[derive(Deserialize)]
pub struct Field {
    pub name: String,
    pub value: String,
}

pub fn parse_logins(json: String) -> Vec<Login> {
    let objs: Vec<BwObject> = serde_json::from_str(&json).expect("JSON was not well formatted");
    objs.into_iter().map(Login::from).collect()
}

pub fn parse_login(json: String) -> Login {
    let obj: BwObject = serde_json::from_str(&json).expect("JSON was not well formatted");
    obj.into()
}
