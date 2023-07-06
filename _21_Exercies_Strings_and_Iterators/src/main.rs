pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut wildcardIdx = 0;
    let request_chars: Vec<char> = request_path.chars().collect();
    let request_len = request_chars.len();
    let mut i = 0;
    for ch in prefix.chars() {
        if request_len == i+wildcardIdx {
            return false
        }
        if ch == '*' {
            while request_chars[i+wildcardIdx] != '/' {
                wildcardIdx += 1;
            }
            wildcardIdx -= 1;
            i += 1;
            continue
        }

        if ch != request_chars[i+wildcardIdx] {
            return false
        }

        i += 1;
    }

    if request_len == i+wildcardIdx {
        return true
    }

    if request_chars[i+wildcardIdx] != '/' {
        return false
    }

    return true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}