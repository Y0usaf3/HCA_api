use hackclub_auth_api::HCAuth;

#[test]
fn test_get_oauth_uri_encoding() {
    let auth = HCAuth::new("client_id", "client_secret", "https://example.com/callback");
    let uri = auth.get_oauth_uri(&["identity", "email"]);
    
    assert!(uri.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fcallback"), "URI should be encoded: {}", uri);
    assert!(uri.contains("scope=identity%20email"), "Scopes should be encoded: {}", uri);
}

#[test]
fn test_get_reauth_uri_encoding() {
    let auth = HCAuth::new("client_id", "client_secret", "https://example.com/callback");
    let uri = auth.get_reauth_uri(Some(3600));
    
    assert!(uri.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fcallback"), "URI should be encoded: {}", uri);
    assert!(uri.contains("max_age=3600"), "Max age should be included: {}", uri);
}
