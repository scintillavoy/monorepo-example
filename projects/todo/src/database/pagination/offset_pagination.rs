use sea_query::{MysqlQueryBuilder, SelectStatement};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::{FromRow, MySqlPool, mysql::MySqlRow};
use utoipa::IntoParams;

use crate::database::pagination::{page::Page, pageable::Pageable};

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct OffsetPagination {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

impl OffsetPagination {
    pub async fn fetch_page<T>(
        &self,
        db: &MySqlPool,
        mut query: SelectStatement,
    ) -> Result<Page<T>, sqlx::Error>
    where
        T: Pageable + for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
    {
        let page = self.page.unwrap_or(0);
        let limit = self.limit.unwrap_or(T::DEFAULT_LIMIT).min(T::MAX_LIMIT);

        query.limit(limit + 1);
        query.offset(page * limit);

        let (sql, values) = query.build_sqlx(MysqlQueryBuilder);

        let mut items: Vec<T> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

        let has_next = items.len() as u64 > limit;
        if has_next {
            items.truncate(limit as usize);
        }

        Ok(Page { items, has_next })
    }
}
