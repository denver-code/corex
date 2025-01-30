use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

/// Defines the interface for extensions that can be registered with the Core system.
/// Extensions must implement this trait to extend the functionality of the Core system.
pub trait ExtensionTrait: Send + Sync {
    /// Returns the name of the extension.
    fn name(&self) -> &'static str;

    /// Extends the provided router with additional routes or middleware.
    fn extend(&self, router: Router) -> Router;
}

/// The Core system manages the router and extensions.
/// It allows registering extensions and running the server.
pub struct CoreX {
    router: Router,
    extensions: Vec<Arc<dyn ExtensionTrait>>,
    host: String,
    port: u16,
}

impl CoreX {
    /// Creates a new Core system with the specified host and port.
    ///
    /// # Arguments
    /// * `host` - The host address to bind the server to (e.g., "127.0.0.1").
    /// * `port` - The port number to bind the server to (e.g., 3000).
    pub fn new(host: String, port: u16) -> Self {
        Self {
            router: Router::new(),
            extensions: Vec::new(),
            host,
            port,
        }
    }

    /// Registers an extension with the Core system.
    ///
    /// # Arguments
    /// * `extension` - An `Arc<dyn ExtensionTrait>` representing the extension to register.
    pub fn register_extension(&mut self, extension: Arc<dyn ExtensionTrait>) {
        self.extensions.push(extension);
    }

    /// Builds the final router by applying all registered extensions.
    pub fn build(self) -> Router {
        let mut router = self.router;
        for extension in self.extensions {
            router = extension.extend(router);
        }
        router
    }

    /// Runs the server and starts listening for incoming requests.
    pub async fn run(self) {
        let addr = format!("{}:{}", self.host, self.port);
        let router = self.build();
        println!("Server running at http://{}", addr);

        let listener = TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Json};
    use serde_json::json;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    /// A test extension that adds a `/test` endpoint.
    struct TestExtension;

    impl ExtensionTrait for TestExtension {
        fn name(&self) -> &'static str {
            "TestExtension"
        }

        fn extend(&self, router: Router) -> Router {
            router.route(
                "/test",
                get(|| async { Json(json!({ "message": "Test endpoint" })) }),
            )
        }
    }

    /// Tests the Core system with a single extension.
    #[tokio::test]
    async fn test_core_with_extension() {
        let mut core = CoreX::new("127.0.0.1".to_string(), 3000);
        core.register_extension(Arc::new(TestExtension));

        // Run the server in the background
        let handle = tokio::spawn(async move {
            core.run().await;
        });

        // Wait for the server to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Test the `/test` endpoint
        let mut stream = TcpStream::connect("127.0.0.1:3000").await.unwrap();
        let request = "GET /test HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
        stream.write_all(request.as_bytes()).await.unwrap();

        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await.unwrap();
        let response = String::from_utf8_lossy(&buffer[..n]);

        assert!(response.contains("Test endpoint"));

        // Shutdown the server
        handle.abort();
    }

    /// Tests the Core system without any extensions.
    #[tokio::test]
    async fn test_core_without_extensions() {
        let core = CoreX::new("127.0.0.1".to_string(), 3001);

        // Run the server in the background
        let handle = tokio::spawn(async move {
            core.run().await;
        });

        // Wait for the server to start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Test the root endpoint (should return 404)
        let mut stream = TcpStream::connect("127.0.0.1:3001").await.unwrap();
        let request = "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
        stream.write_all(request.as_bytes()).await.unwrap();

        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await.unwrap();
        let response = String::from_utf8_lossy(&buffer[..n]);

        assert!(response.contains("404 Not Found"));

        // Shutdown the server
        handle.abort();
    }
}
