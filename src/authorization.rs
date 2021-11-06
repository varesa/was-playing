use std::env;

use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl, basic::BasicClient, reqwest::http_client};
use rocket::http::{ContentType, RawStr};
use rocket::response::Content;
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
pub fn oauth2_callback(code: &RawStr, channel: State<AuthChannel>) -> Content<String> {
    let vars = Vars::get();
    let client = BasicClient::new(
        ClientId::new(vars.client_id),
        Some(ClientSecret::new(vars.client_secret)),
        AuthUrl::new(vars.auth_url).expect("Error building AuthUrl"),
        Some(TokenUrl::new("https://accounts.spotify.com/api/token".into()).expect("Error building TokenUrl"))
    ).set_redirect_uri(RedirectUrl::new("http://localhost:8000/oauth2/callback".into()).expect("Error building RedirectUrl"));

    let access_token = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request(http_client).expect("Failed to get access token");

    channel.channel_tx.lock().expect("Failed to acquire auth TX mutex")
        .send(AuthInfo { access_token: access_token }).expect("Failed to send auth information");

    Content(ContentType::HTML, "<html><body><script type='text/javascript'>window.close();</script></body></html>".into())
}

pub fn get_routes() -> Vec<Route> {
    routes![oauth2_callback]
}


