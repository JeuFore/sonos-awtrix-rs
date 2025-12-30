# Sonos Awtrix RS

A Rust application to bridge Sonos speaker playback information to an Awtrix display via MQTT.

## Features
- Discovers a Sonos speaker by room name
- Publishes current track info and playback progress to Awtrix via MQTT
- Sends notifications to Awtrix when tracks change
- Removes Awtrix app display when playback stops

## Requirements
- Sonos speaker on your local network
- MQTT broker (e.g., Mosquitto)
- Awtrix display (Awtrix Light or compatible)

## Configuration
Set the following environment variables (see `docker-compose.yml` for example):
- `ROOM_NAME`: Name of your Sonos speaker room (e.g., "Living Room Speaker")
- `MQTT_HOST`: MQTT broker URL (e.g., `mqtt://mqtt.example.com:1883`)
- `MQTT_USER`: MQTT username
- `MQTT_PASSWORD`: MQTT password
- `LOG_LEVEL`: (Optional) Log level (`trace`, `debug`, `info`, `warn`, `error`)
- `PORT`: (Optional) HTTP server port (default: 8080)

## Usage
### Docker Compose
```sh
docker-compose up -d
```

### Manual Build & Run
```sh
cargo build --release
ROOM_NAME="Living Room Speaker" MQTT_HOST="mqtt://mqtt.example.com:1883" MQTT_USER="user" MQTT_PASSWORD="pass" ./target/release/sonos-awtrix-rs
```

### Docker
```sh
docker build -t sonos-awtrix-rs .
docker run -e ROOM_NAME="Living Room Speaker" -e MQTT_HOST=... -e MQTT_USER=... -e MQTT_PASSWORD=... sonos-awtrix-rs
```

## How it works
- The app discovers your Sonos speaker by room name.
- It polls the speaker every 5 seconds for playback status and track info.
- When a track is playing, it publishes JSON to MQTT topics for Awtrix to display.
- When playback stops, it removes the Awtrix app display.

## MQTT Topics
- `awtrix/custom/sonos`: Main app display (track info, progress)
- `awtrix/notify`: Notification when track changes

## License
MIT
