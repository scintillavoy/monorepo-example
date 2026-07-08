use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use tracing::{debug, error, instrument};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::app_state::AppState,
    database::pagination::offset_pagination::OffsetPagination,
    network::responses::{ErrorResponse, SuccessResponse},
    todo::todo::{self, CreateTodoRequest, Todo, UpdateTodoRequest},
};

pub const TODO_TAG: &str = "todo";

pub fn router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(list_todos, create_todo))
        .routes(routes!(get_todo, update_todo, delete_todo))
}

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/v1/todos",
    tag = TODO_TAG,
    params(
        OffsetPagination,
    ),
    responses(
        (status = 200, body = SuccessResponse<Vec<Todo>>),
    ),
)]
pub async fn list_todos(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<OffsetPagination>,
) -> Result<Json<SuccessResponse<Vec<Todo>>>, (StatusCode, Json<ErrorResponse>)> {
    match todo::find_all(&state.db, &pagination).await {
        Ok(page) => {
            debug!(
                item_count = page.items.len(),
                has_next = page.has_next,
                "todos fetched"
            );

            Ok(Json(SuccessResponse {
                message: "Success".to_string(),
                contents: page.items,
                has_next: Some(page.has_next),
            }))
        }
        Err(err) => {
            error!(?err, "failed to fetch todos");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to fetch todos".to_string(),
                }),
            ))
        }
    }
}

#[instrument(skip(state))]
#[utoipa::path(
    post,
    path = "/v1/todos",
    tag = TODO_TAG,
    request_body = CreateTodoRequest,
    responses(
        (status = 201, body = SuccessResponse<Todo>),
    ),
)]
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<SuccessResponse<Todo>>), (StatusCode, Json<ErrorResponse>)> {
    match todo::create(&state.db, payload).await {
        Ok(todo) => Ok((
            StatusCode::CREATED,
            Json(SuccessResponse {
                message: "Created".to_string(),
                contents: todo,
                has_next: None,
            }),
        )),
        Err(err) => {
            error!(?err, "failed to create todo");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to create todo".to_string(),
                }),
            ))
        }
    }
}

#[instrument(skip(state))]
#[utoipa::path(
    get,
    path = "/v1/todos/{id}",
    tag = TODO_TAG,
    params(
        ("id" = i64, description = "Todo ID"),
    ),
    responses(
        (status = 200, body = SuccessResponse<Todo>),
        (status = 404, body = ErrorResponse),
    ),
)]
pub async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<SuccessResponse<Todo>>, (StatusCode, Json<ErrorResponse>)> {
    match todo::find_by_id(&state.db, id).await {
        Ok(Some(todo)) => Ok(Json(SuccessResponse {
            message: "Success".to_string(),
            contents: todo,
            has_next: None,
        })),
        Ok(None) => {
            debug!(id, "todo not found");

            Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: format!("Todo with id {} not found", id),
                }),
            ))
        }
        Err(err) => {
            error!(?err, "failed to fetch todo");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to fetch todo".to_string(),
                }),
            ))
        }
    }
}

#[instrument(skip(state))]
#[utoipa::path(
    patch,
    path = "/v1/todos/{id}",
    tag = TODO_TAG,
    params(
        ("id" = i64, description = "Todo ID"),
    ),
    request_body = UpdateTodoRequest,
    responses(
        (status = 200, body = SuccessResponse<Todo>),
        (status = 404, body = ErrorResponse),
    ),
)]
pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<SuccessResponse<Todo>>, (StatusCode, Json<ErrorResponse>)> {
    match todo::update(&state.db, id, payload).await {
        Ok(Some(todo)) => Ok(Json(SuccessResponse {
            message: "Success".to_string(),
            contents: todo,
            has_next: None,
        })),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: format!("Todo with id {} not found", id),
            }),
        )),
        Err(err) => {
            error!(?err, "failed to update todo");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to update todo".to_string(),
                }),
            ))
        }
    }
}

#[instrument(skip(state))]
#[utoipa::path(
    delete,
    path = "/v1/todos/{id}",
    tag = TODO_TAG,
    params(
        ("id" = i64, description = "Todo ID"),
    ),
    responses(
        (status = 200, body = SuccessResponse<bool>),
        (status = 404, body = ErrorResponse),
    ),
)]
pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<SuccessResponse<bool>>, (StatusCode, Json<ErrorResponse>)> {
    match todo::delete(&state.db, id).await {
        Ok(true) => Ok(Json(SuccessResponse {
            message: "Deleted".to_string(),
            contents: true,
            has_next: None,
        })),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: format!("Todo with id {} not found", id),
            }),
        )),
        Err(err) => {
            error!(?err, "failed to delete todo");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Failed to delete todo".to_string(),
                }),
            ))
        }
    }
}
