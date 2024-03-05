use std::collections::{hash_map::Entry, HashMap};

#[derive(Default)]
pub struct Db {
    username_ids: HashMap<String, usize>,
    id_userdatas: Vec<UserData>,
}

struct UserData {
    pwd: String,
    post: Vec<Post>,
}

struct Post {
    header: String,
    date_num: i64,
    content: String,
}

impl Db {
    pub async fn auth(&self, username: &str, pwd: &str) -> Result<(), AuthError> {
        let id = self
            .username_ids
            .get(username)
            .copied()
            .ok_or(AuthError::UserDne)?;

        if pwd == self.id_userdatas[id].pwd {
            Ok(())
        } else {
            Err(AuthError::WrongPwd)
        }
    }

    pub async fn register(&mut self, username: String, pwd: String) -> bool {
        match self.username_ids.entry(username) {
            Entry::Occupied(_) => false,
            Entry::Vacant(entry) => {
                entry.insert(self.id_userdatas.len());
                self.id_userdatas.push(UserData { pwd, post: vec![] });
                true
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Username does not exist")]
    UserDne,
    #[error("Wrong password")]
    WrongPwd,
}
