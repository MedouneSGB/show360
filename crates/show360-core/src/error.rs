use thiserror::Error;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CoreError {
    #[error("salle introuvable: {0}")]
    RoomNotFound(String),

    #[error("salle pleine (max {max})")]
    RoomFull { max: u32 },

    #[error("rôle invalide pour cette opération: {0}")]
    InvalidRole(String),

    #[error("un seul présentateur est autorisé; actuel={current}")]
    PresenterAlreadyAssigned { current: String },

    #[error("code salle invalide")]
    InvalidRoomCode,

    #[error("session inactive")]
    SessionInactive,

    #[error("{0}")]
    Other(String),
}
