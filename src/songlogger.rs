use std::{thread, time::Duration};

use oauth2::TokenResponse;
use reqwest::{blocking::Client, header::AUTHORIZATION};

use crate::AuthInfo;

pub fn run(auth: AuthInfo) {
    println!("Started logger with: {:?}", auth);

    loop {
        let client = Client::new();

        let res = client
            .get("https://api.spotify.com/v1/me/player/currently-playing")
            .header(AUTHORIZATION, format!("Bearer {}", auth.access_token.access_token().secret()))
            .send().expect("Failed to call spotify API");
        println!("{:?}", res.text());

        thread::sleep(Duration::new(1, 0));
    }
}
