use sea_query::{Alias, Expr, MysqlQueryBuilder, Order, SelectStatement};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::{FromRow, MySqlPool, mysql::MySqlRow};
use utoipa::IntoParams;

use crate::database::{
    pagination::{page::Page, pageable::Pageable},
    sorting::sortable::Sortable,
};

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct CursorPagination {
    pub after: Option<u64>,
    pub limit: Option<u64>,
    pub cursor_name: Option<String>,
}

impl CursorPagination {
    pub async fn fetch_page<T>(
        &self,
        db: &MySqlPool,
        mut query: SelectStatement,
    ) -> Result<Page<T>, sqlx::Error>
    where
        T: Pageable + Sortable + for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
    {
        let field = self
            .cursor_name
            .as_ref()
            .and_then(|name| T::resolve_field(name))
            .unwrap_or_else(T::default_sort_field);

        if let Some(after) = self.after {
            query.and_where(Expr::col(Alias::new(field)).gt(after));
        }

        query.order_by(Alias::new(field), Order::Asc);

        let limit = self.limit.unwrap_or(T::DEFAULT_LIMIT).min(T::MAX_LIMIT);
        query.limit(limit + 1);

        let (sql, values) = query.build_sqlx(MysqlQueryBuilder);

        let mut items: Vec<T> = sqlx::query_as_with(&sql, values).fetch_all(db).await?;

        let has_next = items.len() as u64 > limit;
        if has_next {
            items.truncate(limit as usize);
        }

        Ok(Page { items, has_next })
    }
}
