use sea_query::{Alias, Order, SelectStatement};
use serde::Deserialize;

use crate::database::sorting::sortable::Sortable;

#[derive(Debug, Clone, Deserialize)]
pub struct SortQuery {
    pub sort: Option<String>,
}

impl SortQuery {
    pub fn apply<T: Sortable>(&self) -> impl Fn(&mut SelectStatement) {
        move |query: &mut SelectStatement| {
            let mut applied = false;

            if let Some(sort_str) = &self.sort {
                for token in sort_str.split(',') {
                    let (dir, field_name) = if let Some(stripped) = token.strip_prefix('-') {
                        (Order::Desc, stripped)
                    } else {
                        (Order::Asc, token)
                    };

                    if let Some(field) = T::resolve_field(field_name) {
                        query.order_by(Alias::new(field), dir);
                        applied = true;
                    }
                }
            }

            if !applied {
                query.order_by(Alias::new(T::default_sort_field()), Order::Asc);
            }
        }
    }
}
