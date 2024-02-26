use crate::database::prelude::Tasks;
use crate::database::tasks;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode, Json,
};
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
    deleted_at: Option<DateTime<FixedOffset>>,
    user_id: Option<i32>,
}

// localhost:3000/tasks/5
pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&db)
        .await
        .unwrap();
    if let Some(task) = task {
        Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct GetTaskQueryParams {
    priority: Option<String>,
}

// localhost:3000/tasks?priority=A
pub async fn get_all_tasks(
    State(db): State<DatabaseConnection>,
    Query(query_params): Query<GetTaskQueryParams>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut priority_filter = Condition::all();
    if let Some(priority) = query_params.priority {
        priority_filter = if priority.is_empty() {
            priority_filter.add(tasks::Column::Priority.is_null())
        } else {
            priority_filter.add(tasks::Column::Priority.eq(priority))
        };
    }

    let tasks = Tasks::find()
        .filter(priority_filter)
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&db)
        .await
        .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        })
        .collect();
    Ok(Json(tasks))
}
