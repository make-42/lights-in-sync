pub struct Folder {
    pub name: String,
    pub symbol: String,
    pub global_bytes_at_sync_start: i64,
    pub progress: f64,
    pub status: String,
}

impl Folder {
    pub fn new(name: String,symbol: String) -> Folder{
        Folder{
            name: name,
            symbol: symbol,
            global_bytes_at_sync_start:0,
            progress: 1.0,
            status: String::from("unknown"),
        }
    }
    }