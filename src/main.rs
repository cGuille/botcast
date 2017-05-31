extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

fn main() {
    let cfg = Config {
        nickname: Some(String::from("cGuille-botcast")),
        server: Some(String::from("irc.mozilla.org")),
        channels: Some(vec![String::from("#rust-bots")]),
        ..Default::default()
    };

    let server = IrcServer::from_config(cfg).unwrap();
    server.identify().unwrap();

    for message in server.iter() {
        let message = message.unwrap();

        match message.command {
            Command::PRIVMSG(_, ref content) => println!("{}", content),
            _ => (),
        }
    }
}
