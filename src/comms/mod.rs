use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbStatus {
    pub state: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbCompletion {
    pub global_bytes: u64,
    pub need_bytes: u64,
    //pub need_deletes: u64,
    //pub need_items: u64,
    //pub remote_state: String,
}