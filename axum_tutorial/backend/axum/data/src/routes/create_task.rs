use crate::database::prelude::Users;
use crate::database::{tasks, users};
use axum::extract::State;
use axum::{http::StatusCode, Json};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

// We have to move body consuming extractors like Json<RequestTask>
// to the bottom of function signatures, otherwise we will consume the body 
// of the request too early.
pub async fn create_task(
    State(db): State<DatabaseConnection>,
    authorization: TypedHeader<Authorization<Bearer>>,
    Json(req_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    // guard the route with token
    let token = authorization.token();
    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        priority: Set(req_task.priority),
        title: Set(req_task.title),
        description: Set(req_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };
    let res = new_task.save(&db).await.unwrap();
    dbg!(res);
    Ok(())
}
