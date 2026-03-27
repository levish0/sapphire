use sapphire_entity::users::Model as UserModel;

#[derive(Debug)]
pub struct OAuthUserResult {
    /// User model
    pub user: UserModel,
    pub is_new_user: bool,
}
