use crate::stop_token::StopToken;

#[test]
fn stop_token_usage() {
    // Test test basic stop token usage
    let stop_token = StopToken::new();

    // After initialization the token should not request a stop
    assert!(!stop_token.is_stop_requested());
    assert!(stop_token.keep_running());

    // Request stop
    stop_token.request_stop();

    // After stop request we expect to see the stop request
    assert!(stop_token.is_stop_requested());
    assert!(!stop_token.keep_running());
}
