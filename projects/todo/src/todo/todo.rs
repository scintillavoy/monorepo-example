use sea_query::{Alias, Expr, MysqlQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, prelude::FromRow};
use utoipa::ToSchema;

use crate::database::pagination::{
    offset_pagination::OffsetPagination, page::Page, pageable::Pageable,
};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

impl Pageable for Todo {
    const DEFAULT_LIMIT: u64 = 20;
    const MAX_LIMIT: u64 = 100;
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

pub async fn find_all(
    db: &MySqlPool,
    pagination: &OffsetPagination,
) -> Result<Page<Todo>, sqlx::Error> {
    let mut query = Query::select();
    query
        .columns([
            Alias::new("id"),
            Alias::new("title"),
            Alias::new("description"),
            Alias::new("completed"),
        ])
        .from(Alias::new("todo"))
        .order_by(Alias::new("id"), sea_query::Order::Desc);

    pagination.fetch_page::<Todo>(db, query).await
}

pub async fn find_by_id(db: &MySqlPool, id: i64) -> Result<Option<Todo>, sqlx::Error> {
    let (sql, values) = Query::select()
        .columns([
            Alias::new("id"),
            Alias::new("title"),
            Alias::new("description"),
            Alias::new("completed"),
        ])
        .from(Alias::new("todo"))
        .and_where(Expr::col(Alias::new("id")).eq(id))
        .build_sqlx(MysqlQueryBuilder);

    sqlx::query_as_with(&sql, values).fetch_optional(db).await
}

pub async fn create(db: &MySqlPool, payload: CreateTodoRequest) -> Result<Todo, sqlx::Error> {
    let (sql, values) = Query::insert()
        .into_table(Alias::new("todo"))
        .columns([Alias::new("title"), Alias::new("description")])
        .values_panic([payload.title.into(), payload.description.into()])
        .build_sqlx(MysqlQueryBuilder);

    let result = sqlx::query_with(&sql, values).execute(db).await?;
    find_by_id(db, result.last_insert_id() as i64)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update(
    db: &MySqlPool,
    id: i64,
    payload: UpdateTodoRequest,
) -> Result<Option<Todo>, sqlx::Error> {
    let mut query = Query::update();
    query.table(Alias::new("todo"));

    if let Some(title) = payload.title {
        query.value(Alias::new("title"), title);
    }
    if let Some(description) = payload.description {
        query.value(Alias::new("description"), description);
    }
    if let Some(completed) = payload.completed {
        query.value(Alias::new("completed"), completed);
    }

    query
        .value(Alias::new("updated_at"), Expr::current_timestamp())
        .and_where(Expr::col(Alias::new("id")).eq(id));

    let (sql, values) = query.build_sqlx(MysqlQueryBuilder);
    let result = sqlx::query_with(&sql, values).execute(db).await?;
    if result.rows_affected() == 0 {
        return Ok(None);
    }

    find_by_id(db, id).await
}

pub async fn delete(db: &MySqlPool, id: i64) -> Result<bool, sqlx::Error> {
    let (sql, values) = Query::delete()
        .from_table(Alias::new("todo"))
        .and_where(Expr::col(Alias::new("id")).eq(id))
        .build_sqlx(MysqlQueryBuilder);

    let result = sqlx::query_with(&sql, values).execute(db).await?;
    Ok(result.rows_affected() > 0)
}
