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

extern crate rocketchat_client_rs;

use rocketchat_client_rs::RocketClient;
use structopt::StructOpt;

/// A simple command line application to send message to RocketChat via webhook-url.
#[derive(StructOpt, Debug)]
#[structopt(
	about = "A simple command line application to send message to RocketChat via webhook-url.",
)]
struct Opt {
	/// Text message to be sent out.
	#[structopt(short = "t", long = "text")]
	text: String,

	/// Webhook URL to send this text to.
	#[structopt(short = "w", long = "webhook")]
	webhook: String,

	/// Default emoji to show for the bot.
	#[structopt(short = "e", long = "emoji", default_value = ":sos:")]
	emoji: String,

	/// Default username for the bot.
	#[structopt(short = "b", long = "botname", default_value = "superrocket")]
	botname: String,

	/// RocketChat channel to post message to.
	#[structopt(short = "c", long = "channel")]
	channel: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	openssl_probe::init_ssl_cert_env_vars();
	env_logger::init();
	let opt = Opt::from_args();

	let _response = RocketClient::new(opt.webhook)
		.with_channel(opt.channel)
		.with_text(opt.text)
		.with_botname(opt.botname)
		.with_emoji(opt.emoji)
		.with_default_hostname()
		.execute()
		.await?;
	Ok(())
}
