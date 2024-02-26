use crate::database::{prelude::Tasks, tasks, tasks::Model as TaskModel};
use crate::utils::app_error::AppError;
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn find_task_by_id(
    db: &DatabaseConnection,
    task_id: i32,
    user_id: i32,
) -> Result<TaskModel, AppError> {
    Tasks::find_by_id(task_id)
        .filter(tasks::Column::UserId.eq(Some(user_id)))
        .filter(tasks::Column::DeletedAt.is_null())
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting task by id: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your task",
            )
        })?
        .ok_or_else(|| {
            eprintln!("Could not find task by id");
            AppError::new(StatusCode::NOT_FOUND, "not found")
        })
}

pub async fn save_active_task(
    db: &DatabaseConnection,
    task: tasks::ActiveModel,
) -> Result<(), AppError> {
    task.save(db).await.map_err(|error| {
        eprintln!("Error saving task: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving task")
    })?;
    Ok(())
}
