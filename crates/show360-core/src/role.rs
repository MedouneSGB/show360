use serde::{Deserialize, Serialize};

/// Rôles métier d'une session Show360.
///
/// - `Host` : créateur / organisateur de la salle
/// - `Presenter` : unique (P0) — spotlight A/V + screen share
/// - `Participant` : A/V bidirectionnel
/// - `Spectator` : receive-only (chemin grandes salles P1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Host,
    Presenter,
    Participant,
    Spectator,
}

impl Role {
    /// Peut publier caméra / micro.
    pub fn can_publish_av(self) -> bool {
        matches!(self, Self::Host | Self::Presenter | Self::Participant)
    }

    /// Peut publier un partage d'écran (P0 : présentateur uniquement).
    pub fn can_publish_screen(self) -> bool {
        matches!(self, Self::Host | Self::Presenter)
    }

    /// Peut publier un flux 360° (P2 ; flag préparé dès le scaffold).
    pub fn can_publish_projection_360(self) -> bool {
        matches!(self, Self::Host | Self::Presenter)
    }

    /// Peut s'abonner aux tracks des autres.
    pub fn can_subscribe(self) -> bool {
        true
    }

    /// Peut revendiquer le rôle présentateur.
    pub fn can_become_presenter(self) -> bool {
        matches!(self, Self::Host | Self::Participant | Self::Presenter)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::Presenter => "presenter",
            Self::Participant => "participant",
            Self::Spectator => "spectator",
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presenter_can_screen_share() {
        assert!(Role::Presenter.can_publish_screen());
        assert!(!Role::Participant.can_publish_screen());
        assert!(!Role::Spectator.can_publish_av());
    }
}
