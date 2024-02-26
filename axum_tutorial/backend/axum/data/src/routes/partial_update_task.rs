use crate::database::prelude::Tasks;
use crate::database::tasks;
use axum::{extract::{Path, State}, http::StatusCode, Json};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}

pub async fn partial_update(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    // convert db_task into a active model so that we can set things on it
    let mut task = if let Some(db_task) = Tasks::find_by_id(task_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        db_task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(priority) = request_task.priority {
        task.priority = Set(priority);
    }

    if let Some(title) = request_task.title {
        task.title = Set(title);
    }

    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at);
    }

    if let Some(description) = request_task.description {
        task.description = Set(description);
    }

    if let Some(deleted_at) = request_task.deleted_at {
        task.deleted_at = Set(deleted_at);
    }

    Tasks::update(task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
