use sapphire_entity::common::OAuthProvider;

pub trait OAuthProviderConfig {
    const AUTH_URL: &'static str;
    const TOKEN_URL: &'static str;
    const SCOPES: &'static [&'static str];
    const PROVIDER: OAuthProvider;

    fn credentials() -> (&'static str, &'static str, &'static str);
}
