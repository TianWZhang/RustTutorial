use super::{ResponseDataTasks, ResponseTask};
use crate::database::{prelude::Tasks, tasks, users::Model as UserModel};
use crate::utils::app_error::AppError;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetTaskQueryParams {
    priority: Option<String>,
}

// localhost:3000/tasks?priority=A
pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    Query(query_params): Query<GetTaskQueryParams>,
) -> Result<Json<ResponseDataTasks>, AppError> {
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
        .filter(tasks::Column::UserId.eq(user.id))
        .all(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting default tasks: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting default tasks",
            )
        })?
        .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            completed_at: task.completed_at.map(|time| time.to_string()),
        })
        .collect();
    Ok(Json(ResponseDataTasks { data: tasks }))
}
