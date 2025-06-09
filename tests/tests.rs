use p_test::p_test;
use rust_uri::Uri;
use std::str::FromStr;

#[p_test(
    (
        simple,
        "https://example.com",
        Ok(Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: None,
            path: "/".to_string(),
            query: None,
            fragment: None,
        })
    ),
    (
        full,
        "https://example.com:443/path/to?q1=10&q2=20#fragment",
        Ok(Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/path/to".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: Some("fragment".to_string()),
        })
    ),
    (
        no_port,
        "https://example.com/path/to?q1=10&q2=20#fragment",
        Ok(Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: None,
            path: "/path/to".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: Some("fragment".to_string()),
        })
    ),
    (
        no_path,
        "https://example.com:443?q1=10&q2=20#fragment",
        Ok(Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: Some("fragment".to_string()),
        })
    ),
    (
        no_query,
        "https://example.com:443/path/to#fragment",
        Ok(Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/path/to".to_string(),
            query: None,
            fragment: Some("fragment".to_string()),
        })
    ),
    (
        no_fragment,
        "https://example.com:443/path/to?q1=10&q2=20",
        Ok(Uri {
            scheme: "https".to_string(),
            hostname: "example.com".to_string(),
            port: Some(443),
            path: "/path/to".to_string(),
            query: Some("q1=10&q2=20".to_string()),
            fragment: None,
        })
    ),
    (
        no_scheme,
        "///example.com:443/path/to?q1=10&q2=20",
        Err("Scheme not found".to_string()),
    ),
    (
        invalid_delimeters_after_scheme,
        "https:///example.com:443/path/to?q1=10&q2=20",
        Err("Expected hostname, but was /".to_string())
    )
)]
fn test_uri(uri: &str, expected: Result<Uri, String>) {
    assert_eq!(Uri::from_str(uri), expected);
}
