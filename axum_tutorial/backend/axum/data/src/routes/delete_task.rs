use crate::database::prelude::Tasks;
use crate::database::tasks;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
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
    State(db): State<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    // delete by id
    // Tasks::delete_by_id(task_id)
    //     .exec(&db)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if query_params.soft {
        let mut task = if let Some(task) = Tasks::find_by_id(task_id)
            .one(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

        let now = chrono::Utc::now();
        task.deleted_at = Set(Some(now.into()));
        Tasks::update(task)
            .exec(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Tasks::delete_many()
            .filter(tasks::Column::Id.eq(task_id))
            .exec(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}
