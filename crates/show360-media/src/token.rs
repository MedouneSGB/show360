use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use show360_core::{ParticipantId, Role, RoomId};
use thiserror::Error;

use crate::config::LiveKitConfig;
use crate::grants::MediaGrants;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("échec signature JWT: {0}")]
    Jwt(String),
}

/// Claims compatibles LiveKit AccessToken (sous-ensemble utile P0).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub iss: String,
    pub sub: String,
    pub exp: i64,
    pub nbf: i64,
    pub iat: i64,
    pub name: String,
    pub video: VideoGrant,
    /// Métadonnées Show360 (rôle métier) — non lues par LiveKit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoGrant {
    pub room_join: bool,
    pub room: String,
    pub can_publish: bool,
    pub can_subscribe: bool,
    pub can_publish_data: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuedToken {
    /// JWT à passer au SDK LiveKit client.
    pub token: String,
    /// URL WebSocket LiveKit (pas de secret).
    pub livekit_url: String,
    pub room_id: String,
    pub identity: String,
    pub grants: MediaGrants,
    pub expires_at: i64,
}

pub struct TokenIssuer {
    config: LiveKitConfig,
}

impl TokenIssuer {
    pub fn new(config: LiveKitConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &LiveKitConfig {
        &self.config
    }

    /// Émet un token LiveKit pour rejoindre une salle.
    ///
    /// En mode stub / sans LiveKit réel, le JWT reste structurellement valide
    /// pour les clés `devkey`/`secret` du docker-compose local.
    pub fn issue(
        &self,
        room_id: &RoomId,
        identity: &ParticipantId,
        display_name: &str,
        role: Role,
    ) -> Result<IssuedToken, TokenError> {
        let grants = MediaGrants::from_role(role);
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.token_ttl_secs as i64);

        let claims = AccessTokenClaims {
            iss: self.config.api_key.clone(),
            sub: identity.to_string(),
            exp: exp.timestamp(),
            nbf: now.timestamp(),
            iat: now.timestamp(),
            name: display_name.to_string(),
            video: VideoGrant {
                room_join: grants.room_join,
                room: room_id.to_string(),
                can_publish: grants.can_publish,
                can_subscribe: grants.can_subscribe,
                can_publish_data: grants.can_publish_data,
            },
            metadata: Some(
                serde_json::json!({
                    "role": role.as_str(),
                    "can_publish_screen": grants.can_publish_screen,
                    "can_publish_projection_360": grants.can_publish_projection_360,
                    "show360": true,
                })
                .to_string(),
            ),
        };

        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(self.config.api_secret.as_bytes());
        let token = encode(&header, &claims, &key).map_err(|e| TokenError::Jwt(e.to_string()))?;

        Ok(IssuedToken {
            token,
            livekit_url: self.config.url.clone(),
            room_id: room_id.to_string(),
            identity: identity.to_string(),
            grants,
            expires_at: exp.timestamp(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use show360_core::RoomId;

    #[test]
    fn issues_jwt_for_dev_config() {
        let issuer = TokenIssuer::new(LiveKitConfig::dev_self_hosted());
        let room = RoomId::new();
        let identity = ParticipantId::new();
        let issued = issuer
            .issue(&room, &identity, "Alice", Role::Presenter)
            .unwrap();
        assert!(!issued.token.is_empty());
        assert!(issued.grants.can_publish_screen);
        assert_eq!(issued.livekit_url, "ws://127.0.0.1:7880");
    }
}
