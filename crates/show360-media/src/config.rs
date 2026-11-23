use serde::{Deserialize, Serialize};

/// Mode de déploiement LiveKit (dual self-host + cloud).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LiveKitDeployment {
    #[default]
    SelfHosted,
    Cloud,
}

/// Configuration LiveKit lue depuis l'environnement (serveur uniquement).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveKitConfig {
    pub deployment: LiveKitDeployment,
    /// URL WebSocket publique pour les clients (ws/wss).
    pub url: String,
    /// Clé API — secret serveur.
    pub api_key: String,
    /// Secret API — secret serveur.
    pub api_secret: String,
    /// Durée de validité des tokens (secondes).
    pub token_ttl_secs: u64,
}

impl LiveKitConfig {
    /// Charge depuis les variables d'environnement dual-mode.
    ///
    /// - `SHOW360_LIVEKIT_MODE` = `self_hosted` | `cloud` (défaut: self_hosted)
    /// - `SHOW360_LIVEKIT_URL`
    /// - `SHOW360_LIVEKIT_API_KEY`
    /// - `SHOW360_LIVEKIT_API_SECRET`
    /// - `SHOW360_LIVEKIT_TOKEN_TTL` (optionnel, défaut 3600)
    pub fn from_env() -> Result<Self, String> {
        let deployment = match std::env::var("SHOW360_LIVEKIT_MODE")
            .unwrap_or_else(|_| "self_hosted".into())
            .to_ascii_lowercase()
            .as_str()
        {
            "cloud" | "livekit_cloud" => LiveKitDeployment::Cloud,
            _ => LiveKitDeployment::SelfHosted,
        };

        let url = std::env::var("SHOW360_LIVEKIT_URL").unwrap_or_else(|_| match deployment {
            LiveKitDeployment::SelfHosted => "ws://127.0.0.1:7880".into(),
            LiveKitDeployment::Cloud => String::new(),
        });

        let api_key = std::env::var("SHOW360_LIVEKIT_API_KEY").unwrap_or_else(|_| "devkey".into());
        let api_secret =
            std::env::var("SHOW360_LIVEKIT_API_SECRET").unwrap_or_else(|_| "secret".into());

        if deployment == LiveKitDeployment::Cloud && url.is_empty() {
            return Err(
                "SHOW360_LIVEKIT_URL requis en mode cloud (ex. wss://xxx.livekit.cloud)".into(),
            );
        }

        let token_ttl_secs = std::env::var("SHOW360_LIVEKIT_TOKEN_TTL")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600);

        Ok(Self {
            deployment,
            url,
            api_key,
            api_secret,
            token_ttl_secs,
        })
    }

    /// Config de développement locale (docker-compose LiveKit).
    pub fn dev_self_hosted() -> Self {
        Self {
            deployment: LiveKitDeployment::SelfHosted,
            url: "ws://127.0.0.1:7880".into(),
            api_key: "devkey".into(),
            api_secret: "secret".into(),
            token_ttl_secs: 3600,
        }
    }
}
