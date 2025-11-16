# discord-ipc-rust

This library allows you to connect to and control your local Discord client from Rust using the [Discord RPC specification](https://discord.com/developers/docs/topics/rpc).

## Example

The `examples/simple.rs` file contains a basic example of using this crate to authenticate with the RPC server, query the currently selected voice channel, and then mute and unmute the user.

To use it, create an application on the Discord Developer Portal with at least one redirect URI set up (it can just be `http://localhost`). Then, clone this repository, copy the `.env.example` file to `.env` and fill out the variables with those of your application. You can run the example using `cargo run --example simple`.

If an access token is not provided to the example, it will instead demonstrate the process of obtaining an access token using the application's client secret.

## Credits

This library is based on [Hacksore/rpc-discord](https://github.com/Hacksore/rpc-discord), additionally including improvements made in [Deftu/discord-ipc-rust](https://github.com/Deftu/discord-ipc-rust). Missing features and documentation, and code cleanup and refactor, were added by [nekename](https://github.com/nekename), who also updated the library's models to match the 2025-11 version of the Discord RPC docs.
