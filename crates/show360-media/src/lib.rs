//! Couche média Show360 : émission de tokens LiveKit et mapping rôles → grants.
//!
//! Les secrets API LiveKit restent côté serveur (`show360-api`) — jamais exposés au client.

mod config;
mod grants;
mod token;

pub use config::{LiveKitConfig, LiveKitDeployment};
pub use grants::MediaGrants;
pub use token::{AccessTokenClaims, IssuedToken, TokenError, TokenIssuer};
