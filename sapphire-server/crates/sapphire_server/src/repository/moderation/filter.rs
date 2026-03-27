use chrono::{DateTime, Utc};
use sapphire_constants::ModerationAction;
use sapphire_entity::common::ModerationResourceType;
use sapphire_entity::moderation_logs::{
    Column as ModerationLogColumn, Entity as ModerationLogEntity,
};
use sea_orm::{ColumnTrait, QueryFilter, Select};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
/// Filter model for moderation log filter.
pub struct ModerationLogFilter {
    pub actor_id: Option<Uuid>,
    pub resource_type: Option<ModerationResourceType>,
    pub resource_id: Option<Uuid>,
    pub actions: Option<Vec<ModerationAction>>,
    pub created_from: Option<DateTime<Utc>>,
    pub created_to: Option<DateTime<Utc>>,
}

pub(crate) fn apply_moderation_log_filter(
    mut query: Select<ModerationLogEntity>,
    filter: &ModerationLogFilter,
) -> Select<ModerationLogEntity> {
    if let Some(actor_id) = filter.actor_id {
        query = query.filter(ModerationLogColumn::ActorId.eq(actor_id));
    }

    if let Some(resource_type) = &filter.resource_type {
        query = query.filter(ModerationLogColumn::ResourceType.eq(resource_type.clone()));
    }

    if let Some(resource_id) = filter.resource_id {
        query = query.filter(ModerationLogColumn::ResourceId.eq(resource_id));
    }

    if let Some(actions) = &filter.actions
        && !actions.is_empty()
    {
        let action_strs: Vec<&str> = actions.iter().map(|a| a.as_str()).collect();
        query = query.filter(ModerationLogColumn::Action.is_in(action_strs));
    }

    if let Some(created_from) = filter.created_from {
        query = query.filter(ModerationLogColumn::CreatedAt.gte(created_from));
    }

    if let Some(created_to) = filter.created_to {
        query = query.filter(ModerationLogColumn::CreatedAt.lt(created_to));
    }

    query
}
