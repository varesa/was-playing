use std::{thread, time::Duration};

use oauth2::TokenResponse;
use reqwest::{blocking::Client, header::AUTHORIZATION};

use crate::AuthInfo;

use serde::Deserialize;
use serde_json;

#[derive(Debug,Deserialize)]
struct Song {
    name: String,
}

#[derive(Debug,Deserialize)]
struct SpotifyResponse {
    item: Song,
}

pub fn run(auth: AuthInfo) {
    println!("Started logger with: {:?}", auth);

    loop {
        let client = Client::new();

        let res = client
            .get("https://api.spotify.com/v1/me/player/currently-playing")
            .header(AUTHORIZATION, format!("Bearer {}", auth.access_token.access_token().secret()))
            .send().expect("Failed to call spotify API");
        let body = res.text().unwrap();
        let parsed: SpotifyResponse = serde_json::from_str(&body).expect("Failed to parse JSON");
        println!("{:?}", parsed);

        thread::sleep(Duration::new(1, 0));
    }
}
