use std::env;

use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, basic::BasicClient};
use rocket::http::RawStr;
use rocket::Route;
use rocket::State;

use crate::AuthChannel;
use crate::AuthInfo;

struct Vars {
   client_id: String,
   client_secret: String,
   auth_url: String,
}

impl Vars {
    fn get() -> Self {
        Self {
            client_id: env::var("CLIENT_ID").expect("$CLIENT_ID not set"),
            client_secret: env::var("CLIENT_SECRET").expect("$CLIENT_SECRET not set"),
            auth_url: env::var("AUTH_URL").expect("$AUTH_URL not set"),
        }
    }
}

pub fn authenticate() {
    let vars = Vars::get();
    let client = BasicClient::new(
        ClientId::new(vars.client_id),
        Some(ClientSecret::new(vars.client_secret)),
        AuthUrl::new(vars.auth_url).expect("Error building AuthUrl"),
        None
    ).set_redirect_uri(RedirectUrl::new("http://localhost:8000/oauth2/callback".into()).expect("Error building RedirectUrl"));

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user-read-playback-state".into()))
        .url();

    println!("Browse to: {}", auth_url);
}

#[get("/callback?<code>")]
pub fn oauth2_callback(code: &RawStr, channel: State<AuthChannel>) -> &'static str {
    channel.channel_tx.lock().expect("Failed to acquire auth TX mutex")
        .send(AuthInfo { code: code.to_string() }).expect("Failed to send auth information");

    "OK"
}

pub fn get_routes() -> Vec<Route> {
    routes![oauth2_callback]
}


