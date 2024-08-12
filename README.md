Rust Programming Base Template for new Rust Application code.

[YouTube Video](https://www.youtube.com/watch?v=oxx7MmN4Ib0&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

Included packages:

- [thiserror](https://crates.io/crates/thiserror) to easily create error types
- [anyhow](https://crates.io/crates/anyhow) (in `dev`) for more flexible error handling
- [tokio](https://crates.io/crates/tokio) as async runtime

Suggested packages:

- [dotenv](https://crates.io/crates/dotenv) to load `.env` file
- [clap](https://crates.io/crates/clap) to create CLI
- [tracing](https://crates.io/crates/tracing) or [pretty_env_logger](https://crates.io/crates/pretty_env_logger) for logging
- [reqwest](https://crates.io/crates/reqwest) for HTTP requests
- [serde](https://crates.io/crates/serde) with `derive` and [serde_json](https://crates.io/crates/serde_json) for data (de)serialization
- [sqlx](https://crates.io/crates/sqlx) with [sqlite](https://crates.io/crates/sqlx-sqlite) for database access
- [actix-web](https://crates.io/crates/actix-web) as web server and [actix-web-httpauth](https://crates.io/crates/actix-web-httpauth) for HTTP authentication
- [askama](https://crates.io/crates/askama) for type-safe HTML templates and [maud](https://maud.lambda.xyz/) for inline HTML partials
- [futures_util](https://docs.rs/futures-util/latest/futures_util/) async utilities
