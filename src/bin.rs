// MIT License
//
// Copyright (c) 2019 Ankur Srivastava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
#[macro_use]
extern crate clap;
extern crate rocketchat_client_rs;

use clap::{App as ClapApp, Arg};
use rocketchat_client_rs::RocketClient;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    env_logger::init();
    let matches = ClapApp::new("RocketChat-Client")
        .version(crate_version!())
        .author("Ankur Srivastava")
        .about("A simple command line application to send message to RocketChat via webhook-url.")
        .arg(
            Arg::with_name("text")
                .short("t")
                .long("text")
                .takes_value(true)
                .required(true)
                .help("Text message to be sent out."),
        )
        .arg(
            Arg::with_name("webhook")
                .short("w")
                .long("webhook")
                .takes_value(true)
                .required(true)
                .help("Webhook URL to send this text to."),
        )
        .arg(
            Arg::with_name("emoji")
                .short("e")
                .long("emoji")
                .takes_value(true)
                .default_value(":sos:")
                .help("Default emoji to show for the bot."),
        )
        .arg(
            Arg::with_name("botname")
                .short("b")
                .long("botname")
                .takes_value(true)
                .default_value("superrocket")
                .help("Default username for the bot."),
        )
        .arg(
            Arg::with_name("channel")
                .short("c")
                .long("channel")
                .takes_value(true)
                .required(true)
                .help("RocketChat channel to post message to."),
        )
        .get_matches();

    let text = matches.value_of("text").unwrap();
    let webhook = matches.value_of("webhook").unwrap();
    let channel = matches.value_of("channel").unwrap();
    let emoji = matches.value_of("emoji").unwrap().to_owned();
    let botname = matches.value_of("botname").unwrap().to_owned();

    let _response = RocketClient::new(webhook)
        .with_channel(channel)
        .with_text(text)
        .with_botname(botname)
        .with_emoji(emoji)
        .with_default_hostname()
        .execute();
}
