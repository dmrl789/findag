use crate::types::round::RoundInfo;
use sled::Db;

pub struct RoundIndex {
    db: Db,
}

impl RoundIndex {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub fn add_round(&self, round: &RoundInfo) -> Result<(), String> {
        let key = round.round_id.to_le_bytes();
        let value = serde_json::to_vec(round).map_err(|e| e.to_string())?;
        self.db.insert(key, value).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_latest_round(&self) -> Option<RoundInfo> {
        self.db
            .iter()
            .filter_map(|item| {
                item.ok().and_then(|(_, value)| {
                    serde_json::from_slice::<RoundInfo>(&value).ok()
                })
            })
            .max_by_key(|info: &RoundInfo| info.round_id)
    }

    pub fn get_round(&self, round_id: u64) -> Option<RoundInfo> {
        self.db
            .get(round_id.to_le_bytes())
            .ok()
            .flatten()
            .and_then(|value| serde_json::from_slice(&value).ok())
    }
}
