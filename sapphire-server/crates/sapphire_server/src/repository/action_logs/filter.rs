use chrono::{DateTime, Utc};
use sapphire_constants::ActionLogAction;
use sapphire_entity::action_logs::{Column as ActionLogColumn, Entity as ActionLogEntity};
use sapphire_entity::common::ActionResourceType;
use sea_orm::{ColumnTrait, QueryFilter, Select};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct ActionLogFilter {
    pub actor_id: Option<Uuid>,
    pub resource_type: Option<ActionResourceType>,
    pub resource_id: Option<Uuid>,
    pub actions: Option<Vec<ActionLogAction>>,
    pub created_from: Option<DateTime<Utc>>,
    pub created_to: Option<DateTime<Utc>>,
}

pub(crate) fn apply_action_log_filter(
    mut query: Select<ActionLogEntity>,
    filter: &ActionLogFilter,
) -> Select<ActionLogEntity> {
    if let Some(actor_id) = filter.actor_id {
        query = query.filter(ActionLogColumn::ActorId.eq(actor_id));
    }

    if let Some(resource_type) = filter.resource_type {
        query = query.filter(ActionLogColumn::ResourceType.eq(resource_type));
    }

    if let Some(resource_id) = filter.resource_id {
        query = query.filter(ActionLogColumn::ResourceId.eq(resource_id));
    }

    if let Some(actions) = &filter.actions
        && !actions.is_empty()
    {
        let action_strs: Vec<&str> = actions.iter().map(|a| a.as_str()).collect();
        query = query.filter(ActionLogColumn::Action.is_in(action_strs));
    }

    if let Some(created_from) = filter.created_from {
        query = query.filter(ActionLogColumn::CreatedAt.gte(created_from));
    }

    if let Some(created_to) = filter.created_to {
        query = query.filter(ActionLogColumn::CreatedAt.lt(created_to));
    }

    query
}
