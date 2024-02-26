use super::{ResponseDataTask, ResponseTask};
use crate::{
    database::users::Model as UserModel, queries::task_queries, utils::app_error::AppError,
};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sea_orm::DatabaseConnection;

// localhost:3000/tasks/5
pub async fn get_one_task(
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Extension(user): Extension<UserModel>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = task_queries::find_task_by_id(&db, task_id, user.id).await?;

    let response_task = ResponseTask {
        id: task.id,
        title: task.title,
        priority: task.priority,
        description: task.description,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };
    Ok(Json(ResponseDataTask {
        data: response_task,
    }))
}
