# CoreX

**CoreX** is a modular API framework for building extensible systems in Rust. It allows you to create a core system and extend its functionality through plugins (extensions). This crate is designed to be simple, flexible, and easy to use, making it ideal for building modular web applications.

---

## Features

- **Modular Design**: Extend the core system with plugins (extensions).
- **Easy to Use**: Simple API for registering extensions and running the server.
- **Asynchronous**: Built on top of `axum` and `tokio` for high-performance asynchronous operations.
- **Extensible**: Add custom routes, middleware, and functionality through extensions.

---

## Installation

Add `corex-api` to your `Cargo.toml`:

```toml
[dependencies]
corex-api = "0.1.0"
```

---

## Usage

### 1. Define an Extension

Create an extension by implementing the `ExtensionTrait`:

```rust
use axum::{Router, routing::get, response::Json};
use serde_json::json;
use corex_api::{CoreX, ExtensionTrait};
use std::sync::Arc;

struct AuthExtension;

impl ExtensionTrait for AuthExtension {
    fn name(&self) -> &'static str {
        "AuthExtension"
    }

    fn extend(&self, router: Router) -> Router {
        router.route("/auth", get(|| async { Json(json!({ "message": "Auth endpoint" })) }))
    }
}
```

### 2. Register Extensions and Run the Server

Use the `CoreX` system to register extensions and start the server:

```rust
#[tokio::main]
async fn main() {
    let mut core = CoreX::new("127.0.0.1".to_string(), 3000);
    core.register_extension(Arc::new(AuthExtension));
    core.run().await;
}
```

### 3. Test the Endpoints

Start the server and test the endpoints:

```bash
curl http://localhost:3000/auth
```

Response:

```json
{
  "message": "Auth endpoint"
}
```

---

## Example: Multiple Extensions

You can register multiple extensions to extend the core system:

```rust
struct ExampleExtension;

impl ExtensionTrait for ExampleExtension {
    fn name(&self) -> &'static str {
        "ExampleExtension"
    }

    fn extend(&self, router: Router) -> Router {
        router.route("/example", get(|| async { Json(json!({ "message": "Example endpoint" })) }))
    }
}

#[tokio::main]
async fn main() {
    let mut core = Core::new("127.0.0.1".to_string(), 3000);
    core.register_extension(Arc::new(AuthExtension));
    core.register_extension(Arc::new(ExampleExtension));
    core.run().await;
}
```

Test the endpoints:

```bash
curl http://localhost:3000/auth
curl http://localhost:3000/example
```

---

## API Documentation

### `CoreX`

- **`new(host: String, port: u16)`**: Creates a new Core system.
- **`register_extension(extension: Arc<dyn ExtensionTrait>)`**: Registers an extension.
- **`build() -> Router`**: Builds the final router by applying all registered extensions.
- **`run()`**: Starts the server and listens for incoming requests.

### `ExtensionTrait`

- **`name() -> &'static str`**: Returns the name of the extension.
- **`extend(router: Router) -> Router`**: Extends the router with additional routes or middleware.

---

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```

---

## Contributing

Contributions are welcome! If you'd like to contribute, please:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Submit a pull request.

---

## License

This project is licensed under:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))

---

## Acknowledgments

- Built on top of [`axum`](https://github.com/tokio-rs/axum) and [`tokio`](https://github.com/tokio-rs/tokio).
- Inspired by modular and plugin-based architectures.

---

## Questions?

If you have any questions or need help, feel free to open an issue or reach out to the maintainers.

