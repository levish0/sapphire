use crate::repository::user::repository_find_user_by_id;
use crate::repository::user::user_bans::repository_find_user_ban;
use crate::repository::user::user_roles::repository_find_user_roles;
use crate::service::auth::session_types::SessionContext;
use sapphire_entity::common::Role;
use sapphire_errors::errors::Errors;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

/// Permission context for the current actor.
#[derive(Debug, Clone)]
pub struct UserContext {
    pub roles: Vec<Role>,
    pub is_banned: bool,
    pub is_authenticated: bool,
}

impl UserContext {
    pub fn has_role(&self, role: Role) -> bool {
        self.roles.contains(&role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(Role::Admin)
    }

    pub fn require_not_banned(&self) -> Result<(), Errors> {
        if self.is_banned {
            return Err(Errors::UserBanned);
        }
        Ok(())
    }

    pub fn require_role(&self, role: Role) -> Result<(), Errors> {
        self.require_not_banned()?;
        if !self.is_admin() && !self.has_role(role) {
            return Err(Errors::UserPermissionInsufficient);
        }
        Ok(())
    }
}

/// sapphire permission loader and guard helpers.
pub struct PermissionService;

impl PermissionService {
    pub async fn get_context<C>(
        conn: &C,
        session: Option<&SessionContext>,
    ) -> Result<UserContext, Errors>
    where
        C: ConnectionTrait,
    {
        let is_authenticated = session.is_some();

        let (roles, is_banned) = match session {
            Some(session) => {
                let roles = repository_find_user_roles(conn, session.user_id).await?;
                let is_banned = repository_find_user_ban(conn, session.user_id)
                    .await?
                    .is_some();
                (roles, is_banned)
            }
            None => (Vec::new(), false),
        };

        Ok(UserContext {
            roles,
            is_banned,
            is_authenticated,
        })
    }

    pub async fn require_role<C>(
        conn: &C,
        session: Option<&SessionContext>,
        role: Role,
    ) -> Result<UserContext, Errors>
    where
        C: ConnectionTrait,
    {
        let ctx = Self::get_context(conn, session).await?;
        ctx.require_role(role)?;
        Ok(ctx)
    }

    pub async fn require_admin_for_target<C>(
        conn: &C,
        session: Option<&SessionContext>,
        target_user_id: Uuid,
    ) -> Result<UserContext, Errors>
    where
        C: ConnectionTrait,
    {
        Self::require_role_for_target(conn, session, target_user_id, Role::Admin).await
    }

    /// Convenience wrapper for target-scoped moderation gates.
    ///
    /// Both moderators and admins pass this gate. `UserContext::require_role`
    /// only denies actors who are neither admin nor holders of the requested
    /// role.
    pub async fn require_mod_for_target<C>(
        conn: &C,
        session: Option<&SessionContext>,
        target_user_id: Uuid,
    ) -> Result<UserContext, Errors>
    where
        C: ConnectionTrait,
    {
        Self::require_role_for_target(conn, session, target_user_id, Role::Mod).await
    }

    async fn require_role_for_target<C>(
        conn: &C,
        session: Option<&SessionContext>,
        target_user_id: Uuid,
        role: Role,
    ) -> Result<UserContext, Errors>
    where
        C: ConnectionTrait,
    {
        let ctx = Self::require_role(conn, session, role).await?;

        if let Some(session) = session
            && session.user_id == target_user_id
        {
            return Err(Errors::CannotManageSelf);
        }

        repository_find_user_by_id(conn, target_user_id)
            .await?
            .ok_or(Errors::UserNotFound)?;

        let target_roles = repository_find_user_roles(conn, target_user_id).await?;
        if target_roles.contains(&Role::Admin) {
            return Err(Errors::CannotManageHigherOrEqualRole);
        }

        Ok(ctx)
    }
}
