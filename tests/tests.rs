use rust_uri::Uri;
use std::str::FromStr;

#[test]
fn simple() {
    assert_eq!(
        Uri::from_str("https://example.com").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: None,
            path: "/".to_string(),
            query: None,
            fragment: None,
        }
    );
}

#[test]
fn full() {
    assert_eq!(
        Uri::from_str("https://example.com:443/path/to?q1=10&q2=20#fragment").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/path/to".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: Some("fragment".to_string()),
        }
    );
}

#[test]
fn no_port() {
    assert_eq!(
        Uri::from_str("https://example.com/path/to?q1=10&q2=20#fragment").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: None,
            path: "/path/to".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: Some("fragment".to_string()),
        }
    );
}

#[test]
fn no_path() {
    assert_eq!(
        Uri::from_str("https://example.com:443?q1=10&q2=20#fragment").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: Some("fragment".to_string()),
        }
    );
}

#[test]
fn no_query() {
    assert_eq!(
        Uri::from_str("https://example.com:443/path/to#fragment").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/path/to".to_string(),
            query: None,
            fragment: Some("fragment".to_string()),
        }
    );
}

#[test]
fn no_fragment() {
    assert_eq!(
        Uri::from_str("https://example.com:443/path/to?q1=10&q2=20").unwrap(),
        Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/path/to".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: None,
        }
    );
}

#[test]
fn no_scheme() {
    match Uri::from_str("///example.com:443/path/to?q1=10&q2=20") {
        Ok(_) => panic!("expect error, but was ok"),
        Err(e) => assert_eq!("Scheme not found", e),
    }
}

#[test]
fn invalid_delimeters_after_scheme() {
    match Uri::from_str("https:///example.com:443/path/to?q1=10&q2=20") {
        Ok(_) => panic!("expect error, but was ok"),
        Err(e) => assert_eq!("Expected hostname, but was /", e),
    }
}
