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
    pub async fn auth(&self, username: &str, pwd: &str) -> Result<usize, AuthError> {
        let id = self
            .username_ids
            .get(username)
            .copied()
            .ok_or(AuthError::UserDne)?;

        if pwd == self.id_userdatas[id].pwd {
            Ok(id)
        } else {
            Err(AuthError::WrongPwd)
        }
    }

    pub async fn register(
        &mut self,
        username: String,
        pwd: String,
    ) -> Result<usize, RegisterError> {
        match self.username_ids.entry(username) {
            Entry::Occupied(_) => Err(RegisterError),
            Entry::Vacant(entry) => {
                let new_id = self.id_userdatas.len();
                entry.insert(new_id);
                self.id_userdatas.push(UserData { pwd, post: vec![] });
                Ok(new_id)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("user does not exist")]
    UserDne,
    #[error("wrong password")]
    WrongPwd,
}

#[derive(Debug, thiserror::Error)]
#[error("user already exists")]
pub struct RegisterError;
