use crate::database::field_set::FieldSet;

pub trait Sortable: FieldSet {
    fn default_sort_field() -> &'static str;
}
