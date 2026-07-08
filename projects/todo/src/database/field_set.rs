pub trait FieldSet {
    fn resolve_field(name: &str) -> Option<&'static str>;
}
