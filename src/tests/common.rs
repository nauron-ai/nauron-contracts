use uuid::Uuid;

pub fn parse_uuid(value: &str) -> Uuid {
    Uuid::parse_str(value).expect("valid uuid")
}
