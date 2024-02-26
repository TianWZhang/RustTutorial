use crate::{
    database::{prelude::Tasks, tasks, users::Model as UserModel},
    queries::task_queries,
    utils::app_error::AppError,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

// localhost:3000/tasks/4?soft=true
pub async fn delete_task(
    Path(task_id): Path<i32>,
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), AppError> {
    if query_params.soft {
        let mut task = task_queries::find_task_by_id(&db, task_id, user.id)
            .await?
            .into_active_model();
        task.deleted_at = Set(Some(chrono::Utc::now().into()));

        task_queries::save_active_task(&db, task).await
    } else {
        Tasks::delete_many()
            .filter(tasks::Column::Id.eq(task_id))
            .filter(tasks::Column::UserId.eq(user.id))
            .filter(tasks::Column::DeletedAt.is_null())
            .exec(&db)
            .await
            .map_err(|e| {
                eprintln!("Error saving task: {:?}", e);
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving task")
            })?;
        Ok(())
    }
}
