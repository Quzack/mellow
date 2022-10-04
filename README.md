# mellow
 
*Supports [version 10 of the Discord gateway](https://discord.com/developers/docs/reference#api-versioning-api-versions).*

This project was mostly to get better at Rust. I decided against documenting any of the code because it would require me to spend more time writing it than programming. This is not meant to be used for actual projects; instead, use [serenity](https://github.com/serenity-rs/serenity).

Unfinished project, will most likely remain unfinished for a very long time.

## Example

The API was made to be simple, borrowing a few ideas from [discord.js](https://discord.js.org/#/) and a few other wrappers.

```rs
use mellow::{Client, event::Ready};

#[tokio::main]
async fn main() {
    let mut client = Client::from_token("super-secret-token");

    client.on_event::<Ready>(|_, c| {
        let c_user = c.user.as_ref().unwrap();

        println!("{} has successfully launched!", c_user.name);
    });

    if let Err(e) = client.start().await {
        println!("Failed to start client: {}", e);
    }
}
```

## Installation

*Crate has not been deployed to [crates.io](https://crates.io/crates)*

You'll have to add [tokio](https://github.com/tokio-rs/tokio) as a dependency to properly use this crate.

```toml
[dependencies.tokio]
version = "1.20.1"
features = ["macros"]
```

## License

Project uses the [Apache-2.0](LICENSE) license.
