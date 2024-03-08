use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

use indexmap::{indexset, IndexSet};
use serde_json::Value;

#[derive(Default)]
pub struct Db {
    user_table: UserTable,
    post_table: PostTable,
}

#[derive(Default)]
struct UserTable {
    username_id_map: HashMap<Arc<str>, usize>,
    id_users: Vec<User>,
}

#[derive(Default)]
struct PostTable {
    user_id_ids_map: HashMap<usize, IndexSet<usize>>,
    id_posts: Vec<Post>,
}

struct User {
    username: Arc<str>,
    pwd: String,
}

#[derive(Debug, Clone)]
pub enum Post {
    Draft {
        title: String,
        content: Value,
    },
    Published {
        title: String,
        date_num: i64,
        content: Value,
    },
}

#[derive(Debug, Clone, serde::Serialize)]

pub struct Draft {
    header: String,
    content: String,
}

impl Db {
    pub async fn auth(&self, username: &str, pwd: &str) -> Result<usize, AuthError> {
        let id = self
            .user_table
            .username_id_map
            .get(username)
            .copied()
            .ok_or(AuthError::UserDne)?;

        if pwd == self.user_table.id_users[id].pwd {
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
        match self.user_table.username_id_map.entry(username.clone()) {
            Entry::Occupied(_) => Err(RegisterError),
            Entry::Vacant(entry) => {
                let new_id = self.user_table.id_users.len();
                entry.insert(new_id);
                self.user_table.id_users.push(User { username, pwd });
                self.post_table.user_id_ids_map.insert(new_id, indexset![]);
                Ok(new_id)
            }
        }
    }

    pub async fn get_username_by_id(&self, id: usize) -> Option<Arc<str>> {
        self.user_table
            .id_users
            .get(id)
            .map(|userdata| userdata.username.clone())
    }

    pub async fn get_summarized_posts_by_id(
        &self,
        id: usize,
    ) -> Option<impl Iterator<Item = (usize, &Post)>> {
        Some(
            self.post_table
                .user_id_ids_map
                .get(&id)?
                .iter()
                .copied()
                .map(|post_id| (post_id, &self.post_table.id_posts[post_id])),
        )
    }

    pub async fn get_post_by_id_and_post_id(
        &self,
        id: usize,
        post_id: usize,
    ) -> Result<&Post, GetPostByIdAndPostIdError> {
        if !self
            .post_table
            .user_id_ids_map
            .get(&id)
            .ok_or(GetPostByIdAndPostIdError::NoSuchUserId)?
            .contains(&post_id)
        {
            return Err(GetPostByIdAndPostIdError::NoSuchPostId);
        }

        Ok(&self.post_table.id_posts[post_id])
    }

    pub async fn get_post_by_id_and_post_id_mut(
        &mut self,
        id: usize,
        post_id: usize,
    ) -> Result<&mut Post, GetPostByIdAndPostIdError> {
        if !self
            .post_table
            .user_id_ids_map
            .get(&id)
            .ok_or(GetPostByIdAndPostIdError::NoSuchUserId)?
            .contains(&post_id)
        {
            return Err(GetPostByIdAndPostIdError::NoSuchPostId);
        }

        Ok(&mut self.post_table.id_posts[post_id])
    }

    pub async fn new_post_by_id(&mut self, id: usize) -> Option<&mut Post> {
        let post_ids = self.post_table.user_id_ids_map.get_mut(&id)?;

        let new_post_id = self.post_table.id_posts.len();
        self.post_table.id_posts.push(Post::Draft {
            title: "My Post".into(),
            content: "".into(),
        });
        post_ids.insert(new_post_id);
        self.post_table.id_posts.last_mut()
    }

    // pub async fn get_summarized_drafts_by_id(&self, id: usize) -> Option<Vec<Draft>> {
    //     let userdata = self.user_table.id_users.get(id)?;
    //     let mut drafts = userdata.drafts.clone();
    //     for drafts in &mut drafts {
    //         if drafts.content.len() > Self::MAX_SUMMARIZED_LEN {
    //             drafts
    //                 .content
    //                 .truncate(Self::MAX_SUMMARIZED_LEN - Self::TO_BE_CONTINUED.len());
    //             drafts.content.push_str(Self::TO_BE_CONTINUED);
    //         }
    //     }

    //     Some(drafts)
    // }
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

#[derive(Debug, thiserror::Error)]
pub enum GetPostByIdAndPostIdError {
    #[error("user does not exist")]
    NoSuchUserId,
    #[error("post does not exist")]
    NoSuchPostId,
}

impl Post {
    pub fn title_content(&self) -> (&str, &Value) {
        let (Self::Draft { title, content } | Self::Published { title, content, .. }) = self;
        (title, content)
    }

    pub fn title_content_mut(&mut self) -> (&mut String, &mut Value) {
        let (Self::Draft { title, content } | Self::Published { title, content, .. }) = self;
        (title, content)
    }

    pub fn publish(&mut self, date_num: i64) {
        if let Self::Draft { title, content } = self {
            let title = std::mem::take(title);
            let content = std::mem::take(content);
            *self = Self::Published {
                title,
                date_num,
                content,
            }
        }
    }
}
