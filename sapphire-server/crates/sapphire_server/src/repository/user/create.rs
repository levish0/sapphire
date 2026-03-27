use sapphire_entity::users::{ActiveModel as UserActiveModel, Model as UserModel};
use sapphire_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use crate::utils::crypto::password::hash_password;

///
///
/// - `service_create_user`
///
/// # Errors
pub async fn repository_create_user<C>(
    conn: &C,
    email: String,
    handle: String,
    display_name: String,
    password: String,
) -> Result<UserModel, Errors>
where
    C: ConnectionTrait,
{
    let hashed_password = hash_password(&password)?;

    repository_create_user_with_password_hash(conn, email, handle, display_name, hashed_password)
        .await
}

/// Creates a user record with a pre-hashed password.
pub async fn repository_create_user_with_password_hash<C>(
    conn: &C,
    email: String,
    handle: String,
    display_name: String,
    password_hash: String,
) -> Result<UserModel, Errors>
where
    C: ConnectionTrait,
{
    let new_user = UserActiveModel {
        id: Default::default(),
        display_name: Set(display_name),
        handle: Set(handle),
        bio: Set(None),
        email: Set(email),
        password: Set(Some(password_hash)),
        profile_image: Set(None),
        banner_image: Set(None),
        totp_secret: Set(None),
        totp_enabled_at: Set(None),
        totp_backup_codes: Set(None),
        created_at: Default::default(),
    };

    let user = new_user.insert(conn).await?;

    Ok(user)
}
