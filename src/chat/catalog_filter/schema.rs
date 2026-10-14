use serde_json::{Value, json};

use super::MAX_DATA_FILTERS;

pub fn catalog_filter_array_schema() -> Value {
    json!({
        "type": "array", "maxItems": MAX_DATA_FILTERS,
        "description": "contains matches text or text list elements, case-insensitively.",
        "items": {
            "type": "object", "properties": {
                "field": {"type": "string"},
                "operator": {"enum": ["equal", "contains", "greater_or_equal", "less_or_equal", "is_null"]},
                "value": {"type": ["string", "number", "boolean", "array", "object", "null"]}
            },
            "required": ["field", "operator"], "additionalProperties": false,
            "if": {"properties": {"operator": {"const": "is_null"}}},
            "then": {"properties": {"value": {"type": "null"}}},
            "else": {"required": ["value"]}
        }
    })
}
