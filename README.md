# Stream VOD Recorder

`svr` is a utility that monitors and downloads livestreams for any site with a youtube-dl extractor.

## Example CLI Usage
```bash
svr -c /path/to/config.toml
```

## Example `config.toml`:
```toml
output = "/media/livestreams"

[[streams]]
subpath = "ThePrimeagen"
type = "youtube-dl"
url = "https://twitch.tv/ThePrimeagen"
frequency = "10m"

[[streams]]
subpath = "LofiGirl"
type = "youtube-dl"
url = "https://www.youtube.com/@LofiGirl/live"
frequency = "2h"

[[streams]]
subpath = "LofiGirl"
type = "youtube-dl"
url = "https://www.youtube.com/watch?v=jfKfPfyJRdk"
frequency = "2h"
```

## Example `docker-compose.yml`:
```yaml
svr:
  container_name: svr
  image: git.nickzana.dev/nick/svr:latest
  volumes:
    - $CONFIG/config.toml:/config.toml
    - $MEDIA_PATH/livestreams:/media/livestreams
  restart: unless-stopped
```

## TODO
- More configuration options for individual streams
- Handle server/stream interruptions (e.g. merging multiple stream files)
- Monitor playlists and other non-live content (monitor a channel???)
- Automatic deletion after e.g. a period of time
- Improved logging (tracing?)
- Save metadata somewhere
- Add --help menu
