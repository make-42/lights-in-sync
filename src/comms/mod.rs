use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbStatus {
    pub state: String,
    pub in_sync_bytes: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbCompletion {
    pub global_bytes: i64,
    pub need_bytes: i64,
    //pub need_deletes: u64,
    //pub need_items: u64,
    //pub remote_state: String,
}