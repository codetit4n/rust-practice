use serde::Serialize;
use std::collections::HashMap;
use uuid;

// Make sure all fields are serialized
#[derive(Debug, Serialize)]
struct Account {
    id: uuid::Uuid,
    user_name: String,
    last_login_at: Option<String>,
}

impl Account {
    fn to_json_manually(&self) -> String {
        format!(
            r#"{{ "id": "{:?}" , "user_name": {:?} , "last_login_at": {:?}}}"#,
            self.id,
            self.user_name,
            self.last_login_at.clone().unwrap()
        )
    }
    fn to_json_serde(&self) -> String {
        // then call serde_json::to_string() on it
        serde_json::to_string(self).unwrap()
    }

    // to avoid serializing fields
    fn to_json_serde_hashmap(&self) -> String {
        let mut out: HashMap<&'static str, String> = HashMap::new();
        out.insert("id", self.id.to_string());
        out.insert("user_name", self.user_name.clone());
        if let Some(login) = &self.last_login_at {
            out.insert("last_login_at", login.clone());
        }
        serde_json::to_string(&out).unwrap()
    }
}

fn main() {
    let acct = Account {
        id: uuid::Uuid::new_v4(),
        user_name: "lokesh".to_owned(),
        last_login_at: Some("06-09-2023".to_owned()),
    };

    println!("Manually: {}", acct.to_json_manually());
    println!("Serde: {}", acct.to_json_serde());
    println!("Serde and Hashmap: {}", acct.to_json_serde_hashmap());
}
