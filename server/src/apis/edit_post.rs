use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{db::GetPostByIdAndPostIdError, session::Session};

use super::Model;

pub(crate) fn router(state: Model) -> Router<Model> {
    Router::new()
        .route("/requestedit/:post_id", post(request_edit))
        .route("/newpost", post(new_post))
        .route("/savechange/:post_id", post(save_change))
}

#[debug_handler]
async fn request_edit(
    State(model): State<Model>,
    session: Session,
    Path(post_id): Path<usize>,
) -> Result<Json<Value>, Error> {
    let db = model.db.read().await;
    let (title, content) = db
        .get_post_by_id_and_post_id(session.user_id(), post_id)
        .await?
        .title_content();

    Ok(Json(json!({
        "title":title,
        "content":content,
    })))
}

// TODO: receive the post also
#[debug_handler]
async fn new_post(State(model): State<Model>, session: Session) -> Result<Json<Value>, Error> {
    let mut db = model.db.write().await;
    let (title, content) = db
        .new_post_by_id(session.user_id())
        .await
        .ok_or(Error::InvalidUserId)?
        .title_content();

    Ok(Json(json!({
        "title": title,
        "content": content,
    })))
}

#[debug_handler]
async fn save_change(
    State(model): State<Model>,
    session: Session,
    Path(post_id): Path<usize>,
    Json(payload): Json<PostPayload>,
) -> Result<(), Error> {
    let mut db = model.db.write().await;
    let (title, content) = db
        .get_post_by_id_and_post_id_mut(session.user_id(), post_id)
        .await?
        .title_content_mut();

    *title = payload.title;
    *content = payload.content;

    Ok(())
}

#[derive(Deserialize)]
struct PostPayload {
    title: String,
    content: Value,
}

// #[debug_handler]
// pub(crate) async fn handler(
//     State(model): State<Model>,
//     session: Session,
//     Path(kind): Path<Kind>,
// ) -> Result<Json<Value>, Error> {
//     match kind {
//         Kind::RequestEdit { post_id } => {
//             let db = model.db.read().await;
//             let (title, content) = db
//                 .get_post_by_id_and_post_id(session.user_id(), post_id)
//                 .await?
//                 .title_content();

//             Ok(Json(json!({
//                 "title":title,
//                 "content":content,
//             })))
//         }
//         Kind::SaveChange => {
//             let mut db = model.db.write().await;
//             todo!()
//         }
//         Kind::NewPost => {
//             let mut db = model.db.write().await;
//             let (title, content) = db
//                 .new_post_by_id(session.user_id())
//                 .await
//                 .ok_or(Error::InvalidUserId)?
//                 .title_content();

//             Ok(Json(json!({
//                 "title":title,
//                 "content":content,
//             })))
//         }
//     }
// }

// #[derive(Deserialize)]
// #[cfg_attr(test, derive(serde::Serialize))]
// #[serde(rename_all = "lowercase")]
// pub(crate) enum Kind {
//     RequestEdit { post_id: usize },
//     SaveChange,
//     NewPost,
// }

pub(crate) enum Error {
    InvalidUserId,
    InvalidPostId,
}

impl From<GetPostByIdAndPostIdError> for Error {
    fn from(e: GetPostByIdAndPostIdError) -> Self {
        match e {
            GetPostByIdAndPostIdError::NoSuchUserId => Self::InvalidUserId,
            GetPostByIdAndPostIdError::NoSuchPostId => Self::InvalidPostId,
        }
    }
}

impl Error {
    const INVALID_USER_ID: &'static str = "Invalid user id";
    const INVALID_POST_ID: &'static str = "Invalid post id";
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InvalidUserId => {
                (StatusCode::UNAUTHORIZED, Self::INVALID_USER_ID).into_response()
            }
            Self::InvalidPostId => (StatusCode::BAD_REQUEST, Self::INVALID_POST_ID).into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use crate::apis::edit_post::Kind;

    // #[test]
    // fn what_kind_serde_into() {
    //     println!(
    //         "{:?}",
    //         serde_json::ser::to_string_pretty(&Kind::RequestEdit { post_id: 3 })
    //     )
    // }
}
