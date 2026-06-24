#[cfg(test)]
mod tests {
    use crate::json_util::{string_list_from_json, string_list_to_json};

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
}
