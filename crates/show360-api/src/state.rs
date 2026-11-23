use std::collections::HashMap;
use std::sync::Arc;

use show360_core::{Room, RoomCode, RoomId};
use show360_media::TokenIssuer;
use tokio::sync::RwLock;

use crate::config::ApiConfig;

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub config: ApiConfig,
    pub issuer: TokenIssuer,
    pub rooms: RwLock<HashMap<String, Room>>,
    pub codes: RwLock<HashMap<String, String>>,
}

impl AppState {
    pub fn new(config: ApiConfig) -> Self {
        let issuer = TokenIssuer::new(config.livekit.clone());
        Self {
            inner: Arc::new(AppStateInner {
                config,
                issuer,
                rooms: RwLock::new(HashMap::new()),
                codes: RwLock::new(HashMap::new()),
            }),
        }
    }

    pub async fn insert_room(&self, room: Room) {
        let id = room.id.to_string();
        let code = room.code.to_string();
        self.inner.rooms.write().await.insert(id.clone(), room);
        self.inner.codes.write().await.insert(code, id);
    }

    pub async fn get_room_by_id(&self, id: &RoomId) -> Option<Room> {
        self.inner.rooms.read().await.get(&id.0).cloned()
    }

    pub async fn get_room_by_code(&self, code: &RoomCode) -> Option<Room> {
        let codes = self.inner.codes.read().await;
        let id = codes.get(code.as_str())?;
        self.inner.rooms.read().await.get(id).cloned()
    }

    pub async fn update_room<F, T>(&self, id: &RoomId, f: F) -> Option<T>
    where
        F: FnOnce(&mut Room) -> T,
    {
        let mut rooms = self.inner.rooms.write().await;
        let room = rooms.get_mut(&id.0)?;
        Some(f(room))
    }
}
