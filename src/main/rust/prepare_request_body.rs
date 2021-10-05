use std::collections::HashMap;

const PID: &'static str = &"DK5QPID";

pub fn prepare_request_body(key_x: i32, key_y: i32, color: &str) -> HashMap<&str, String> {
    let location_str = format!("{},{}", key_x, key_y);

    let mut body_fields = HashMap::new();
    body_fields.insert("pid", PID.to_string());
    body_fields.insert("zoneId", location_str);
    body_fields.insert("color", color.to_string());
    body_fields.insert("effect", "SET_COLOR".to_string());

    return body_fields;
}
