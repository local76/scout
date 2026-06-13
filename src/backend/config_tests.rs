use super::*;
use std::path::PathBuf;

#[derive(Default, Clone, Debug, PartialEq)]
struct TestFields {
    pub value: String,
    pub number: i32,
}

impl ConfigFields for TestFields {
    fn parse_field(&mut self, key: &str, val: &str) {
        match key {
            "value" => self.value = val.to_string(),
            "number" => self.number = val.parse().unwrap_or(0),
            _ => {}
        }
    }
    fn serialize_fields(&self) -> Vec<(String, String)> {
        vec![
            ("value".to_string(), self.value.clone()),
            ("number".to_string(), self.number.to_string()),
        ]
    }
}

#[test]
fn test_config_path() {
    let app_name = "test-app-123";
    let filename = "config-123.yaml";
    let path = AppConfig::<TestFields>::config_path(app_name, filename);
    assert!(path.is_some());
    let path = path.unwrap();
    assert!(path.to_string_lossy().contains("local76"));
    assert!(path.to_string_lossy().contains(app_name));
    assert!(path.to_string_lossy().contains(filename));
}

#[test]
fn test_write_file_atomic() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let test_dir = PathBuf::from(manifest_dir).join("target").join("test_scratch");
    std::fs::create_dir_all(&test_dir).unwrap();
    let file_path = test_dir.join("atomic_test.txt");

    let content = b"hello world atomic";
    write_file_atomic(&file_path, content).unwrap();

    assert!(file_path.exists());
    let read_content = std::fs::read(&file_path).unwrap();
    assert_eq!(read_content, content);

    // Clean up
    let _ = std::fs::remove_file(&file_path);
}

#[test]
fn test_app_config_load_nonexistent() {
    // Loading from a nonexistent path should fall back to default
    let app_name = "nonexistent-app-unique-987";
    let filename = "nonexistent-config.yaml";
    let cfg = AppConfig::<TestFields>::load(app_name, filename);
    assert_eq!(cfg.fields, TestFields::default());
}
