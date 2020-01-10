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

use hostname;
use reqwest;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
/// RocketClient represents a minimal rocketchat client which can be used to send text
/// messages to a given webhook.
///
/// # Examples
/// Basic usage:
///
/// ```rust ignore
/// use rocketchat_client_rs::RocketClient;
/// let _response = RocketClient::new("www.facebook.com")
///     	.with_channel("#test-logs")
///             .execute();
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RocketClient {
	text: String,
	webhook: String,
	emoji: Option<String>,
	botname: Option<String>,
	channel: Option<String>,
	hostname: Option<OsString>,
}

impl RocketClient {
	pub fn new<T>(webhook: T) -> Self
	where
		T: Into<String>,
	{
		RocketClient {
			webhook: webhook.into(),
			..Default::default()
		}
	}

	/// Define the channel name where this message is to be sent
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust ignore
	/// use rocketchat_client_rs::RocketClient;
	/// let _response = RocketClient::new("www.facebook.com")
	///     	.with_channel("#test-logs")
	///             .execute();
	/// ```
	pub fn with_channel<T>(&mut self, channel: T) -> &mut Self
	where
		T: Into<String>,
	{
		self.channel = Some(channel.into());
		self
	}

	/// Define the hostname from which this message is being sent
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust ignore
	/// use rocketchat_client_rs::RocketClient;
	/// let _response = RocketClient::new("www.facebook.com")
	///     	.with_hostname("my-machine")
	///             .execute();
	/// ```
	pub fn with_hostname<T>(&mut self, hostname: T) -> &mut Self
	where
		T: Into<OsString>,
	{
		self.hostname = Some(hostname.into());
		self
	}

	/// Define text which is to be sent.
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust ignore
	/// use rocketchat_client_rs::RocketClient;
	/// let _response = RocketClient::new("www.facebook.com")
	/// 	.with_channel("#test-logs")
	/// 	.with_text("Hi world")
	///     .execute();
	/// ```
	pub fn with_text<T>(&mut self, text: T) -> &mut Self
	where
		T: Into<String>,
	{
		self.text = text.into();
		self
	}

	/// Add default hostname with the message being sent
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust ignore
	/// use rocketchat_client_rs::RocketClient;
	/// let _response = RocketClient::new("www.facebook.com")
	/// 	.with_channel("#test-logs")
	/// 	.with_default_hostname()
	///     .execute();
	/// ```
	pub fn with_default_hostname(&mut self) -> &mut Self {
		self.hostname = hostname::get().ok();

		self
	}

	/// Add botname with the message being sent
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust ignore
	/// use rocketchat_client_rs::RocketClient;
	/// let _response = RocketClient::new("www.facebook.com")
	/// 	.with_channel("#test-logs")
	/// 	.with_botname("some-bot")
	///     .execute();
	/// ```
	pub fn with_botname<T>(&mut self, botname: T) -> &mut Self
	where
		T: Into<Option<String>>,
	{
		self.botname = botname.into();
		self
	}

	/// Add emoji with the message being sent
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust ignore
	/// use rocketchat_client_rs::RocketClient;
	/// let _response = RocketClient::new("www.facebook.com")
	/// 	.with_channel("#test-logs")
	/// 	.with_emoji(":sos:")
	///     .execute();
	/// ```
	pub fn with_emoji<T>(&mut self, emoji: T) -> &mut Self
	where
		T: Into<Option<String>>,
	{
		self.emoji = emoji.into();
		self
	}

	/// Execute the builder pattern finally after constructing the object.
	///
	/// # Examples
	/// Basic usage:
	///
	/// ```rust ignore
	/// use rocketchat_client_rs::RocketClient;
	/// let _response = RocketClient::new("www.facebook.com")
	/// 	.with_channel("#test-logs")
	/// 	.with_text("Hi world")
	/// 	.with_default_hostname()
	///     .execute();
	/// ```
	pub async fn execute(&mut self) -> Result<(), reqwest::Error> {
		let rocket_client = RocketClient {
			webhook: self.webhook.clone(),
			channel: self.channel.clone(),
			hostname: self.hostname.clone(),
			emoji: self.emoji.clone(),
			botname: self.botname.clone(),
			text: self.text.clone(),
		};

		reqwest::Client::new()
			.post(&rocket_client.webhook)
			.json(&rocket_client)
			.send()
			.await?
			.json()
			.await?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_execute() {
		let response = RocketClient::new("https://example.com")
			.with_channel("#test-logs")
			.with_text("Hi world")
			.with_default_hostname()
			.execute();

		assert!(
			response.is_err(),
			"Should return error in case unable to send text to Rocketchat"
		);
	}
}
