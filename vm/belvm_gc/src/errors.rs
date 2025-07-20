#[derive(thiserror::Error, Debug, PartialEq)]
pub enum MemoryError {
    #[error("allocation failed")]
    AllocationFailed,
}
