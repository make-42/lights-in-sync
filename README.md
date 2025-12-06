# Lights in Sync
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

# Folder list
[[folders]]
id = "xxxxx-xxxxx"
icon = " "

[[folders]]
id = "xxxxx-xxxxx"
icon = " "
```
