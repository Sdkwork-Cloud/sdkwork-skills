#[cfg(test)]
mod tests {
    use crate::json_util::{
        json_value_from_text, json_value_to_text, string_list_from_json, string_list_to_json,
    };

    #[test]
    fn string_list_json_roundtrip() {
        let items = vec!["cap.demo.run".to_string(), "cap.demo.read".to_string()];
        let encoded = string_list_to_json(&items, "capabilities").expect("encode");
        let decoded = string_list_from_json(encoded.as_str(), "capabilities").expect("decode");
        assert_eq!(items, decoded);
    }

    #[test]
    fn string_list_from_json_rejects_non_array() {
        string_list_from_json("{}", "tags").expect_err("object must fail");
    }

    #[test]
    fn json_value_text_roundtrip_preserves_object_shape() {
        let value = serde_json::json!({"type": "object", "required": ["path"]});
        let encoded = json_value_to_text(&value, "schema").expect("encode JSON value");
        let decoded = json_value_from_text(&encoded, "schema").expect("decode JSON value");
        assert_eq!(decoded, value);
    }
}
