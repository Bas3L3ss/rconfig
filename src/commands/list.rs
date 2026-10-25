use crate::{ file::{ FileError, open }, parser::parse, tokenizer::tokenize };

pub fn list(path: &str) -> Result<(), FileError> {
    let contents = open(path)?;
    let tokens = tokenize(&contents);
    let list = parse(&contents, &tokens);

    println!("{list:#?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::value::Value;

    #[test]
    fn test_list_contains_all_keys_from_challenge_fixture() {
        let fixture_path = "fixtures/challenge.rcfg";
        let contents = open(fixture_path).expect("fixture should open");
        let tokens = tokenize(&contents);
        let list = parse(&contents, &tokens);

        // The outermost value should be a struct (HashMap)
        let top = &list;

        // Check top-level keys
        let expected_top_keys = [
            "app_name",
            "environment",
            "debug",
            "maintenance_mode",
            "port",
            "max_connections",
            "retry_count",
            "temperature_offset",
            "request_timeout",
            "negative_threshold",
            "database_url",
            "welcome_message",
            "empty_string",
            "features",
            "allowed_ports",
            "feature_flags",
            "temperature_values",
            "database",
            "logging",
            "cache",
            "version",
        ];

        for key in expected_top_keys.iter() {
            assert!(top.contains_key(key), "Missing key: {}", key);
        }

        // Check nested struct `database`
        if let Some(Value::Struct(database)) = top.get("database") {
            let expected_db_keys = ["host", "port", "username", "password", "connection"];
            for key in expected_db_keys.iter() {
                assert!(database.contains_key(key), "Missing database field: {}", key);
            }
            if let Some(Value::Struct(conn)) = database.get("connection") {
                let expected_conn_keys = ["max_connections", "min_connections", "timeout"];
                for key in expected_conn_keys.iter() {
                    assert!(conn.contains_key(key), "Missing connection field: {}", key);
                }
            } else {
                panic!("database.connection missing or not a struct");
            }
        } else {
            panic!("database missing or not a struct");
        }

        // Check nested struct `logging`
        if let Some(Value::Struct(logging)) = top.get("logging") {
            let expected_logging_keys = ["level", "file", "rotation"];
            for key in expected_logging_keys.iter() {
                assert!(logging.contains_key(key), "Missing logging field: {}", key);
            }
            if let Some(Value::Struct(rotation)) = logging.get("rotation") {
                let expected_rotation_keys = ["enabled", "max_files"];
                for key in expected_rotation_keys.iter() {
                    assert!(rotation.contains_key(key), "Missing rotation field: {}", key);
                }
            } else {
                panic!("logging.rotation missing or not a struct");
            }
        } else {
            panic!("logging missing or not a struct");
        }

        // Check nested struct `cache`
        if let Some(Value::Struct(cache)) = top.get("cache") {
            let expected_cache_keys = ["enabled", "ttl"];
            for key in expected_cache_keys.iter() {
                assert!(cache.contains_key(key), "Missing cache field: {}", key);
            }
        } else {
            panic!("cache missing or not a struct");
        }
    }
}
