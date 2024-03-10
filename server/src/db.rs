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
        user_id: usize,
        title: String,
        content: Value,
    },
    Published {
        user_id: usize,
        title: String,
        date_num: u128,
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
    ) -> Result<&Post, ByIdAndPostIdError> {
        if !self
            .post_table
            .user_id_ids_map
            .get(&id)
            .ok_or(ByIdAndPostIdError::NoSuchUserId)?
            .contains(&post_id)
        {
            return Err(ByIdAndPostIdError::NoSuchPostId);
        }

        Ok(&self.post_table.id_posts[post_id])
    }

    pub async fn get_post_by_id_and_post_id_mut(
        &mut self,
        id: usize,
        post_id: usize,
    ) -> Result<&mut Post, ByIdAndPostIdError> {
        if !self
            .post_table
            .user_id_ids_map
            .get(&id)
            .ok_or(ByIdAndPostIdError::NoSuchUserId)?
            .contains(&post_id)
        {
            return Err(ByIdAndPostIdError::NoSuchPostId);
        }

        Ok(&mut self.post_table.id_posts[post_id])
    }

    pub async fn new_post_by_id(&mut self, id: usize, post: Post) -> Option<&mut Post> {
        let post_ids = self.post_table.user_id_ids_map.get_mut(&id)?;

        let new_post_id = self.post_table.id_posts.len();
        self.post_table.id_posts.push(post);
        post_ids.insert(new_post_id);
        self.post_table.id_posts.last_mut()
    }

    pub async fn delete_post_by_id_and_post_id(
        &mut self,
        id: usize,
        post_id: usize,
    ) -> Result<(), ByIdAndPostIdError> {
        if self
            .post_table
            .user_id_ids_map
            .get_mut(&id)
            .ok_or(ByIdAndPostIdError::NoSuchUserId)?
            .shift_remove(&post_id)
        {
            Ok(())
        } else {
            Err(ByIdAndPostIdError::NoSuchPostId)
        }
    }

    pub async fn change_pwd_by_id(
        &mut self,
        id: usize,
        reenter_pwd: &str,
        new_pwd: String,
    ) -> Result<(), ChangePwdByIdError> {
        let user = self
            .user_table
            .id_users
            .get_mut(id)
            .ok_or(ChangePwdByIdError::NoSuchUserId)?;

        if user.pwd != reenter_pwd {
            return Err(ChangePwdByIdError::MismatchPwd);
        }

        user.pwd = new_pwd;
        Ok(())
    }

    pub async fn search_post<'a>(
        &'a self,
        keyword: &'a str,
        since: Option<u128>,
    ) -> impl Iterator<Item = (&str, &Post)> + 'a {
        self.post_table
            .user_id_ids_map
            .values()
            .flatten()
            .map(|&post_id| &self.post_table.id_posts[post_id])
            .flat_map(|post| {
                self.user_table
                    .id_users
                    .get(post.user_id())
                    .map(|user| (&*user.username, post))
            })
            .filter(move |&(_, post)| post.published_and_contains_since(keyword, since))
    }

    pub async fn search_post_by_author<'a>(
        &'a self,
        keyword: &'a str,
        author: &'a str,
        since: Option<u128>,
    ) -> impl Iterator<Item = &Post> + 'a {
        self.search_post_by_author_impl(keyword, author, since)
            .into_iter()
            .flatten()
    }

    fn search_post_by_author_impl<'a>(
        &'a self,
        keyword: &'a str,
        author: &'a str,
        since: Option<u128>,
    ) -> Option<impl Iterator<Item = &Post> + 'a> {
        let id = self.user_table.username_id_map.get(author)?;
        Some(
            self.post_table.user_id_ids_map[id]
                .iter()
                .copied()
                .map(|post_id| &self.post_table.id_posts[post_id])
                .filter(move |&post| post.published_and_contains_since(keyword, since)),
        )
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

#[derive(Debug, thiserror::Error)]
pub enum ByIdAndPostIdError {
    #[error("user does not exist")]
    NoSuchUserId,
    #[error("post does not exist")]
    NoSuchPostId,
}

#[derive(Debug, thiserror::Error)]
pub enum ChangePwdByIdError {
    #[error("user does not exist")]
    NoSuchUserId,
    #[error("reenter password does not match with the old password")]
    MismatchPwd,
}

impl Post {
    pub fn user_id(&self) -> usize {
        let (Self::Draft { user_id, .. } | Self::Published { user_id, .. }) = self;
        *user_id
    }

    pub fn title_content(&self) -> (&str, &Value) {
        let (Self::Draft { title, content, .. } | Self::Published { title, content, .. }) = self;
        (title, content)
    }

    pub fn title_content_mut(&mut self) -> (&mut String, &mut Value) {
        let (Self::Draft { title, content, .. } | Self::Published { title, content, .. }) = self;
        (title, content)
    }

    pub fn publish(&mut self, date_num: u128) {
        if let Self::Draft {
            title,
            content,
            user_id,
        } = self
        {
            let title = std::mem::take(title);
            let content = std::mem::take(content);
            *self = Self::Published {
                title,
                date_num,
                content,
                user_id: *user_id,
            }
        }
    }

    fn published_and_contains_since(&self, needle: &str, since: Option<u128>) -> bool {
        fn content_to_iter(content: &Value) -> Option<impl IntoIterator<Item = &str>> {
            let Value::Object(content) = content else {
                return None;
            };

            let content = content.get("ops")?;

            let Value::Array(content) = content else {
                return None;
            };

            Some(content.iter().filter_map(|piece| {
                let Value::Object(piece) = piece else {
                    return None;
                };
                let piece = piece.get("insert")?;
                let Value::String(piece) = piece else {
                    return None;
                };
                Some(&piece[..])
            }))
        }

        let Self::Published {
            title,
            date_num,
            content,
            ..
        } = self
        else {
            return false;
        };

        if matches!(since, Some(since) if since > *date_num) {
            return false;
        }

        content_to_iter(content)
            .into_iter()
            .flatten()
            .chain([&title[..]])
            .any(|string| string.contains(needle))
    }
}
