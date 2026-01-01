# Lights in Sync
<img width="106" height="37" alt="image" src="https://github.com/user-attachments/assets/bcc4bf5b-adb2-4382-a3b1-f2ea2884cc03" />

A topbar program for getting and displaying Syncthing folder statuses.


## Configuration
Config file at `~/.config/ontake/lights-in-sync/config.toml`

Example usage:
```toml
# API Key
api_key = ""

# Colors
idle_color = "#00FF00"
scanning_color = "#FFFF00"
error_color = "#FF0000"
paused_color = "#BBBBBB"

# Refresh period / dynamic refresh period
refresh_millis = 2000
dynamic_refresh_millis = 500

# Folder list
[[folders]]
id = "xxxxx-xxxxx"
icon = " "

[[folders]]
id = "xxxxx-xxxxx"
icon = " "
```
