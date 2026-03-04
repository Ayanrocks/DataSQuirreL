pub trait DbTypeMapper: Send + Sync {
    fn sql_to_js_type(&self, sql_type: &str) -> String;
    fn cast_to_text_expr(&self, column_name: &str, raw_type: &str) -> String;
}
