use soroban_env_host::events::HostEvent;

pub fn events(events: &[HostEvent]) {
    for event in events {
        tracing::info!(log = event.to_string());
    }
}
