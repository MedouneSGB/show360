use serde::{Deserialize, Serialize};
use show360_core::Role;

/// Permissions LiveKit dérivées du rôle métier Show360.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaGrants {
    pub room_join: bool,
    pub can_publish: bool,
    pub can_subscribe: bool,
    pub can_publish_data: bool,
    /// Publication screen share (présentateur / host).
    pub can_publish_screen: bool,
    /// Préparé pour P2 — flux 360°.
    pub can_publish_projection_360: bool,
}

impl MediaGrants {
    pub fn from_role(role: Role) -> Self {
        Self {
            room_join: true,
            can_publish: role.can_publish_av(),
            can_subscribe: role.can_subscribe(),
            can_publish_data: role.can_publish_av(),
            can_publish_screen: role.can_publish_screen(),
            can_publish_projection_360: role.can_publish_projection_360(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spectator_is_receive_only() {
        let g = MediaGrants::from_role(Role::Spectator);
        assert!(!g.can_publish);
        assert!(g.can_subscribe);
        assert!(!g.can_publish_screen);
    }

    #[test]
    fn presenter_can_screen_and_360_flags() {
        let g = MediaGrants::from_role(Role::Presenter);
        assert!(g.can_publish);
        assert!(g.can_publish_screen);
        assert!(g.can_publish_projection_360);
    }
}
