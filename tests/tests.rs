use p_test::p_test;
use rust_uri::Uri;
use std::str::FromStr;

#[p_test(
    ("https://example.com"),
    ("https://example.com:443/path/to?q1=10&q2=20#fragment"),
    ("https://example.com/path/to?q1=10&q2=20#fragment"),
    ("https://example.com:443?q1=10&q2=20#fragment"),
    ("https://example.com:443/path/to#fragment"),
    ("https://example.com:443/path/to?q1=10&q2=20"),
    ("https://example.com:443/path/to??q1=10&q2=20##frag"),
)]
fn test_uri_display(uri: &str) {
    let parsed_uri = Uri::from_str(uri).expect("Failed to parse URI");
    assert_eq!(uri, parsed_uri.to_string());
}
