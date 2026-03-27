pub enum SignInResult {
    Success(String), // session_id

    PendingSignup {
        pending_token: String,
        email: String,
        display_name: String,
    },
}
