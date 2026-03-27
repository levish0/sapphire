pub use sea_orm_migration::prelude::*;

mod common;

mod m20260324_061519_user_role_enum;
mod m20260324_061602_users;
mod m20260324_061633_user_roles;
mod m20260324_061641_user_bans;
mod m20260324_061651_oauth_providers;
mod m20260324_061710_oauth_connections;
mod m20260324_062030_user_preferences;
mod m20260324_063449_create_post;
mod m20260324_063459_create_comment;
mod m20260324_063504_action_resource_type_enum;
mod m20260324_063600_moderation_resource_type_enum;
mod m20260324_063700_create_action_logs;
mod m20260324_063800_create_moderation_logs;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260324_061519_user_role_enum::Migration),
            Box::new(m20260324_061602_users::Migration),
            Box::new(m20260324_061633_user_roles::Migration),
            Box::new(m20260324_061641_user_bans::Migration),
            Box::new(m20260324_061651_oauth_providers::Migration),
            Box::new(m20260324_061710_oauth_connections::Migration),
            Box::new(m20260324_062030_user_preferences::Migration),
            Box::new(m20260324_063449_create_post::Migration),
            Box::new(m20260324_063459_create_comment::Migration),
            Box::new(m20260324_063504_action_resource_type_enum::Migration),
            Box::new(m20260324_063600_moderation_resource_type_enum::Migration),
            Box::new(m20260324_063700_create_action_logs::Migration),
            Box::new(m20260324_063800_create_moderation_logs::Migration),
        ]
    }
}
