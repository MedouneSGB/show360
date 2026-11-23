use show360_media::LiveKitConfig;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub livekit: LiveKitConfig,
    pub public_base_url: String,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self, String> {
        let livekit = LiveKitConfig::from_env()?;
        let public_base_url = std::env::var("SHOW360_PUBLIC_BASE_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:8080".into());
        Ok(Self {
            livekit,
            public_base_url,
        })
    }
}
