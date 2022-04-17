worker-logger
=============

Logger implementation for Cloudflare Workers.
Bridges the [`log`](https://crates.io/crates/log) ecosystem to Cloudflare Worker.

# Example

Initialize the logger with a string. This crate uses the same filter syntax as
[`env_logger`](https://crates.io/crates/env_logger):

```rust
worker_logger::init_with_string("info");
```

For more details, please visit <https://docs.rs/env_logger/latest/env_logger/#enabling-logging>

Or initialize with a set level:

```rust
use log::Level;
worker_logger::init_with_level(&Level::Debug);
```

Or with a Cloudflare Worker environment variable:

```rust
worker_logger::init_with_env(env, "LOG");
```
