#![feature(proc_macro_hygiene, decl_macro)]

mod authorization;
mod songlogger;

use std::{sync::{Mutex, mpsc::{Receiver, Sender, channel}}, thread};

use crate::authorization::{authenticate, get_routes as auth_routes};
use crate::songlogger::run as run_songlogger;

#[macro_use] extern crate rocket;

#[derive(Debug)]
pub struct AuthInfo {
    code: String,
}

pub struct AuthChannel {
    channel_tx: Mutex<Sender<AuthInfo>>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {

    let (tx, rx): (Sender<AuthInfo>, Receiver<AuthInfo>) = channel();
    thread::spawn(move || {
        loop {
            authenticate();    
            println!("Logger thread: waiting for auth info");
            let info = rx.recv().expect("Failed to read auth info from channel");
            run_songlogger(info);
        }
    });

    rocket::ignite()
        .manage(AuthChannel { channel_tx: Mutex::new(tx) })
        .mount("/", routes![index])
        .mount("/oauth2", auth_routes())
        .launch();
}
