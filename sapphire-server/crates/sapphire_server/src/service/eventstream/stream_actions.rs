use crate::state::EventStreamSender;
use axum::response::sse::Event;
use sapphire_dto::action_logs::{ActionLogResponse, StreamActionsQuery};
use std::convert::Infallible;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

/// Build a filtered action-log SSE stream for sapphire's current resource model.
pub fn service_stream_actions(
    eventstream_tx: EventStreamSender,
    params: StreamActionsQuery,
) -> impl futures::Stream<Item = Result<Event, Infallible>> {
    let rx = eventstream_tx.subscribe();

    BroadcastStream::new(rx)
        .filter_map(move |result| match result {
            Ok(event) => {
                if let Some(user_id) = params.user_id
                    && event.actor_id != Some(user_id)
                {
                    return None;
                }

                if let Some(resource_type) = &params.resource_type
                    && &event.resource_type != resource_type
                {
                    return None;
                }

                if let Some(resource_id) = params.resource_id
                    && event.resource_id != Some(resource_id)
                {
                    return None;
                }

                if let Some(ref actions) = params.actions
                    && !actions.iter().any(|action| action.as_str() == event.action)
                {
                    return None;
                }

                Some(event)
            }
            Err(_) => None,
        })
        .map(|event: ActionLogResponse| {
            Ok(Event::default()
                .event("action_log")
                .id(event.id.to_string())
                .data(serde_json::to_string(&event).unwrap_or_default()))
        })
}
