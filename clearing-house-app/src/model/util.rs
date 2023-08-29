/// Returns a new UUID as a string with hyphens.
pub fn new_uuid() -> String {
    use uuid::Uuid;
    Uuid::new_v4().hyphenated().to_string()
}
