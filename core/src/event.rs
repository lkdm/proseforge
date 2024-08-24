enum ResourceStatus {
    Pending,
    Loaded,
    Error,
}

// Listen to events use async/await.
#[derive(Debug)]
pub enum CoreEvent {
    ResourceStatus,
}
