use std::{thread, time::Duration};

use oauth2::TokenResponse;
use reqwest::{blocking::Client, header::AUTHORIZATION};

use crate::AuthInfo;

use serde::Deserialize;
use serde_json;

#[derive(Debug,Deserialize)]
struct Album {
    name: String,
}

#[derive(Debug,Deserialize)]
struct Artist {
    name: String,
}

#[derive(Debug,Deserialize)]
struct Song {
    name: String,
    album: Album,
    artists: Vec<Artist>,
    uri: String,
}

#[derive(Debug,Deserialize)]
struct SpotifyResponse {
    timestamp: u64,
    item: Song,
}

pub fn run(auth: AuthInfo) {
    println!("Started logger with: {:?}", auth);

    let mut last_uri = String::from("");

    loop {
        let client = Client::new();

        let res = client
            .get("https://api.spotify.com/v1/me/player/currently-playing")
            .header(AUTHORIZATION, format!("Bearer {}", auth.access_token.access_token().secret()))
            .send().expect("Failed to call spotify API");

        if res.status() == 204 {
            // Nothing playing
            continue;
        }

        let body = res.text().unwrap();
        let parsed: SpotifyResponse = serde_json::from_str(&body).expect("Failed to parse JSON");

        if parsed.item.uri != last_uri {
            println!("New song: {:?}", parsed);
            last_uri = parsed.item.uri;
        }

        thread::sleep(Duration::new(30, 0));
    }
}
