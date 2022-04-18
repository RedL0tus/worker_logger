worker_logger
=============

Logger implementation for Cloudflare Workers.
Bridges the [`log`](https://crates.io/crates/log) ecosystem to Cloudflare Worker.

Example
-------

Initialize the logger with a string:

```rust
worker_logger::init_with_string("info");
```

Or initialize with a level struct:

```rust
use log::Level;
worker_logger::init_with_level(&Level::Debug);
```

Or with a Cloudflare Worker environment variable:

```rust
worker_logger::init_with_env(env, "LOG")?;
```

Features
--------

 - `env_logger_string`: Enables advanced logging filters. Uses the same syntax as
   [`env_logger`](https://crates.io/crates/env_logger). For more details, please visit
   <https://docs.rs/env_logger/latest/env_logger/#enabling-logging>.