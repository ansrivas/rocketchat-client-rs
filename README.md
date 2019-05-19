# rocketchat-client-rs

[![Build Status](https://dev.azure.com/asrivascs/asrivascs/_apis/build/status/ansrivas.rocketchat-client-rs?branchName=master)](https://dev.azure.com/asrivascs/asrivascs/_build/latest?definitionId=2&branchName=master)
[![Version info](https://img.shields.io/crates/v/rocketchat_client_rs.svg)](https://crates.io/crates/rocketchat_client_rs)
[![Docs.rs](https://docs.rs/rocketchat_client_rs/badge.svg)](https://docs.rs/rocketchat_client_rs)


Send notifications/messages to RocketChat from command-line or from inside your code.

## Installation

### Using cargo

```bash
cargo install rocketchat_client_rs
```

## Test

To run the tests:

`make test`

## Usage

## Examples
- As a command line tool:
  ```bash
  $ rocketchat-client --channel "#test-logs" --webhook "https://blah.at.blah-blah-blah.com" --text "hi"
  ```

- As a library:
  ```rust
  use rocketchat_client_rs::RocketClient;

  fn main() {
  let _response = RocketClient::new("https://blah.at.blah-blah-blah.com")
      .with_channel("#test-logs")
      .with_text("Hi world")
      .with_default_hostname()
      .execute();
  }
  ```


## Documentation
  ```
  $ cargo doc --no-deps
  ```

## License
MIT

## Credits
The Azure pipeline template has been taken from https://github.com/graphql-rust/juniper here.
