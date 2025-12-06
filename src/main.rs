mod comms;
mod folder;
mod utils;
mod config;

use reqwest::Client;
use comms::{DbStatus,DbCompletion};

use config::load_config;

use std::{thread, time};

use utils::progress_string;

use crate::config::load_folders_from_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mut folders_list = load_folders_from_config(&config)?;

    loop {
        let mut out_of_sync = false;
        let mut global_bytes_at_sync_start: i64 = 0;
        let mut global_bytes_current: i64 = 0;
        let mut global_bytes_at_sync_end: i64 = 0;
        let mut progress: f64 = 1.; // 0 -> 1
    for folder in &mut folders_list {
    let client = Client::new();
    let api_key = config.api_key.as_str();

    let url = format!(
        "http://localhost:8384/rest/db/status?folder={}",
        folder.name
    );

    let mut status: DbStatus = client
        .get(url)
        .header("X-API-Key", api_key)
        .send()
        .await?
        // .error_for_status()?   // turn HTTP errors into Rust errors
        .json()
        .await.unwrap_or(DbStatus { state: String::from(""),in_sync_bytes:0 });

    let url = format!(
        "http://localhost:8384/rest/db/completion?folder={}",
        folder.name
    );

    if status.state == "" {
        status.state = String::from("paused")
    }

    let completion = if status.state == "paused"{
        DbCompletion { global_bytes: 0, need_bytes: 0 }
    } else {
    client
        .get(url)
        .header("X-API-Key", api_key)
        .send()
        .await?
        // .error_for_status()?   // turn HTTP errors into Rust errors
        .json()
        .await.unwrap_or(DbCompletion { global_bytes: 0, need_bytes: 0 })
    };


    folder.status = status.state.clone();

    if status.state == "idle" || status.state == "scanning" || status.state == "paused"{
        folder.global_bytes_at_sync_start = completion.global_bytes-completion.need_bytes;
        folder.progress = 1.;
    } else {
        out_of_sync = true;
        let folder_global_bytes_at_sync_start = folder.global_bytes_at_sync_start;
    let folder_global_bytes_current = status.in_sync_bytes;
    let folder_global_bytes_at_sync_end = completion.global_bytes;
    if folder_global_bytes_at_sync_end-folder_global_bytes_at_sync_start > 0{
        folder.progress = (folder_global_bytes_current as f64 -folder_global_bytes_at_sync_start as f64)/(folder_global_bytes_at_sync_end as f64-folder_global_bytes_at_sync_start as f64)
    }
    }

    global_bytes_at_sync_start += folder.global_bytes_at_sync_start;
    global_bytes_current += status.in_sync_bytes;
    global_bytes_at_sync_end += completion.global_bytes;
}
if global_bytes_at_sync_end-global_bytes_at_sync_start > 0{
    progress = (global_bytes_current as f64 -global_bytes_at_sync_start as f64)/(global_bytes_at_sync_end as f64-global_bytes_at_sync_start as f64)
}

    let mut out_strings: Vec<String> =vec![];
    for folder in &mut folders_list {
        let mut folder_string = folder.symbol.clone();
        match folder.status.as_str() {
             "error" => folder_string.push_str(format!("<span color='{}'></span>",&config.error_color).as_str()) ,
             "scanning" => folder_string.push_str(format!("<span color='{}'></span>",&config.scanning_color).as_str()) ,
             "paused" => folder_string.push_str(format!("<span color='{}'></span>",&config.paused_color).as_str()) ,
             _ => folder_string.push_str(format!("<span color='{}'>{}</span>",&config.idle_color,progress_string(folder.progress)).as_str()),
        }
        out_strings.push(folder_string);
    }
    if out_of_sync {
        out_strings.push(format!("{} %",(progress*100.) as u8));
    }

    let out = out_strings.join(" ");
    println!("{}",out);

    if out_of_sync{
    thread::sleep(time::Duration::from_millis(config.dynamic_refresh_millis));} else {
    thread::sleep(time::Duration::from_millis(config.refresh_millis));
    }
}
}
