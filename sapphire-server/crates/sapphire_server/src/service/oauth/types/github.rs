use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubUserInfo {
    /// GitHub user ID
    pub id: u64,
    /// GitHub username (login)
    pub login: String,
    /// Full name (optional)
    pub name: Option<String>,
    /// Email address (optional, may need separate API call)
    pub email: Option<String>,
    /// Avatar/profile image URL
    pub avatar_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubEmail {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
}
