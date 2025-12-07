use dotenv::dotenv;
use env_logger::Env;
use log::{debug, error, info};
use paho_mqtt as mqtt;
use serde::{Deserialize, Serialize};
use std::{env, process, time::Duration};

#[derive(Serialize, Deserialize, Clone)]
struct AwtrixNotify {
    text: String,
    icon: String,
    progress: u8,
}

#[tokio::main]
async fn main() -> Result<(), sonor::Error> {
    dotenv().ok();

    env_logger::Builder::from_env(Env::new().filter_or("LOG_LEVEL", "info")).init();

    let room_name = env::var("ROOM_NAME").expect("ROOM_NAME not set");

    let speaker = match sonor::find(&room_name, Duration::from_secs(3)).await? {
        Some(speaker) => speaker,
        None => {
            error!("speaker '{}' doesn't exist", room_name);
            process::exit(1);
        }
    };

    info!("Found speaker: {}", speaker.name().await?);

    let mqtt = init_mqtt().await;

    let mut awtrix_notify = AwtrixNotify {
        text: String::new(),
        icon: String::from("sonos"),
        progress: 0,
    };

    let mut previous_track = String::new();
    let mut app_is_remove = false;

    loop {
        let is_playing = speaker.is_playing().await?;

        if is_playing {
            let track_info = speaker.track().await?;

            if let Some(track_info) = track_info {
                let current_track = track_info.track();

                let duration = track_info.duration();
                let elapsed = track_info.elapsed();

                info!(
                    "Currently playing: {} - Elapsed: {}/{}",
                    current_track,
                    fmt_duration(elapsed),
                    fmt_duration(duration)
                );

                awtrix_notify.text = format!("{}", current_track);
                awtrix_notify.progress = (track_info.elapsed() * 100 / track_info.duration()) as u8;

                let json = serde_json::to_string(&awtrix_notify)
                    .expect("Failed to serialize awtrix_notify");

                debug!("JSON: {}", json);

                let msg = mqtt::Message::new("awtrix/custom/sonos", json.clone(), mqtt::QOS_0);
                mqtt.publish(msg);

                if current_track.title().to_string() != previous_track || app_is_remove {
                    info!("Sending notification...");
                    let msg = mqtt::Message::new("awtrix/notify", json, mqtt::QOS_0);
                    mqtt.publish(msg);
                }

                if app_is_remove {
                    info!("Adding awtrix app...");
                    app_is_remove = false;
                }

                previous_track = current_track.title().to_string();
            } else {
                debug!("No track are currently playing...");

                if !app_is_remove {
                    app_is_remove = true;
                    remove_awtrix_app(&mqtt);
                }
            }
        } else {
            if !app_is_remove {
                app_is_remove = true;
                remove_awtrix_app(&mqtt);
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

fn fmt_duration(secs: u32) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}

fn remove_awtrix_app(mqtt: &mqtt::AsyncClient) {
    info!("Removing awtrix app...");
    let msg = mqtt::Message::new("awtrix/custom/sonos", "{}", mqtt::QOS_0);
    mqtt.publish(msg);
}

async fn init_mqtt() -> mqtt::AsyncClient {
    let cli = mqtt::AsyncClient::new(env::var("MQTT_HOST").expect("MQTT_HOST not set"))
        .expect("Failed to create client");

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .user_name(env::var("MQTT_USER").expect("MQTT_USER not set"))
        .password(env::var("MQTT_PASSWORD").expect("MQTT_PASSWORD not set"))
        .finalize();

    cli.connect(conn_opts).await.expect("Failed to connect");

    info!("Connected to MQTT broker!");

    return cli;
}
