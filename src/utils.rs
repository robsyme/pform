pub fn mask_auth_header(value: &str) -> String {
    if let Some(_token) = value.strip_prefix("Bearer ") {
        "Bearer $TOWER_ACCESS_TOKEN".to_string()
    } else {
        value.to_string()
    }
}
