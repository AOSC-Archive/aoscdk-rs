fn is_valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.starts_with('-') {
        return false;
    }
    for c in hostname.as_bytes() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-' {
            continue;
        } else {
            return false;
        }
    }

    true
}

fn is_acceptable_username(username: &str) -> bool {
    if username.is_empty() || username.starts_with('-') {
        return false;
    }
    for c in username.as_bytes() {
        if c.is_ascii_whitespace() || *c == b'/' || *c == b'\\' || *c == b':' {
            return false;
        }
    }

    true
}

#[test]
fn test_hostname_validation() {
    assert_eq!(is_valid_hostname("foo"), true);
    assert_eq!(is_valid_hostname("foo-2e10"), true);
    assert_eq!(is_valid_hostname("jeffbai-device"), true);
    assert_eq!(is_valid_hostname("invalid_host"), false);
    assert_eq!(is_valid_hostname("-invalid"), false);
    assert_eq!(is_valid_hostname("+invalid"), false);
}

#[test]
fn test_username_validation() {
    assert_eq!(is_acceptable_username("foo"), true);
    assert_eq!(is_acceptable_username("老白"), true);
    assert_eq!(is_acceptable_username("/root"), false);
    assert_eq!(is_acceptable_username("root:root"), false);
    assert_eq!(is_acceptable_username("root\n"), false);
    assert_eq!(is_acceptable_username("root\t"), false);
    assert_eq!(is_acceptable_username("ro ot"), false);
}
