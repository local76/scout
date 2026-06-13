use super::*;

#[test]
fn test_identity_username() {
    let name = username();
    assert!(!name.is_empty(), "Username should not be empty");
}

#[test]
fn test_identity_hostname() {
    let host = hostname();
    assert!(!host.is_empty(), "Hostname should not be empty");
}

#[test]
fn test_identity_user_host() {
    let combined = user_host();
    assert!(combined.contains('@'), "user_host should contain '@'");
    assert!(combined.starts_with(&username()), "user_host should start with username");
    assert!(combined.ends_with(&hostname()), "user_host should end with hostname");
}

#[test]
fn test_identity_shell_name() {
    let shell = shell_name();
    assert!(!shell.is_empty(), "Shell name should not be empty");
}
