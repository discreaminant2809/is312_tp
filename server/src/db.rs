use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

#[derive(Default)]
pub struct Db {
    username_ids: HashMap<Arc<str>, usize>,
    id_userdatas: Vec<UserData>,
}

struct UserData {
    username: Arc<str>,
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
        let username: Arc<str> = username.into();
        match self.username_ids.entry(username.clone()) {
            Entry::Occupied(_) => Err(RegisterError),
            Entry::Vacant(entry) => {
                let new_id = self.id_userdatas.len();
                entry.insert(new_id);
                self.id_userdatas.push(UserData {
                    username,
                    pwd,
                    post: vec![],
                });
                Ok(new_id)
            }
        }
    }

    pub async fn get_username_from_id(&self, id: usize) -> Option<Arc<str>> {
        self.id_userdatas
            .get(id)
            .map(|userdata| userdata.username.clone())
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
