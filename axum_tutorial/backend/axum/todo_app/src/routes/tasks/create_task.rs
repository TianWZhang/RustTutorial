use super::create_task_extractor::ValidateCreateTask;
use super::{ResponseDataTask, ResponseTask};
use crate::database::{tasks, users::Model as UserModel};
use crate::utils::app_error::AppError;
use axum::extract::State;
use axum::Extension;
use axum::{http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

// We have to move body consuming extractors like Json<RequestTask> or ValidateCreateTask (which implements FromRequest trait)
// to the bottom of function signatures, otherwise we will consume the body
// of the request too early.
pub async fn create_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    task: ValidateCreateTask,
) -> Result<(StatusCode, Json<ResponseDataTask>), AppError> {
    let new_task = tasks::ActiveModel {
        priority: Set(task.priority),
        title: Set(task.title.unwrap()),
        description: Set(task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };
    let active_task = new_task.save(&db).await.map_err(|e| {
        eprintln!("Error saving task: {:?}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving task")
    })?;
    let task = active_task.try_into_model().map_err(|e| {
        eprintln!("Error converting task active model to model: {:?}", e);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })?;

    let response_task = ResponseTask {
        id: task.id,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task.completed_at.map(|time| time.to_string()),
    };

    Ok((
        StatusCode::CREATED,
        Json(ResponseDataTask {
            data: response_task,
        }),
    ))
}
