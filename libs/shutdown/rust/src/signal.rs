use std::io;

use tokio_util::sync::CancellationToken;
use tracing::info;

/// Installs OS signal handlers for graceful shutdown and returns a
/// [`CancellationToken`] that is cancelled when a shutdown signal is received.
///
/// This function synchronously registers signal listeners for:
///
/// - `SIGINT` (Ctrl+C)
/// - `SIGTERM` (on Unix platforms)
///
/// If signal registration fails, an [`io::Error`] is returned.
///
/// Once a shutdown signal is received, the returned [`CancellationToken`]
/// is cancelled, allowing the application to coordinate a graceful shutdown across
/// multiple components (for example, an HTTP server, Kafka consumer, background tasks, etc.).
///
/// The returned [`CancellationToken`] can be cloned and shared across
/// independent tasks to implement a unified shutdown sequence.
///
/// # Runtime requirement
///
/// This function must be called when a Tokio runtime is available.
/// Although signal registration itself is synchronous, the function
/// internally uses `tokio::spawn` to await signals asynchronously.
/// It should therefore be invoked inside a `#[tokio::main]` context or
/// from code already running within a Tokio runtime.
///
/// # Installation order and signal safety
///
/// This function should be called as early as possible during application
/// startup, before performing any dangerous or non-idempotent initialization
/// (for example, starting servers, spawning background workers, acquiring distributed
/// locks, running migrations, or consuming messages).
///
/// If a shutdown signal is delivered *before* the handlers are installed,
/// the OS default behavior applies and the process may terminate immediately,
/// bypassing graceful shutdown logic. Installing the handlers early minimizes
/// the window in which the process could exit abruptly.
///
/// # Examples
///
/// Using with an Axum HTTP server:
///
/// ```no_run
/// use tokio_util::sync::CancellationToken;
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     let shutdown = install()?;
///
///     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
///         .await
///         .unwrap();
///
///     axum::serve(listener, axum::Router::new())
///         .with_graceful_shutdown(async move {
///             shutdown.cancelled().await;
///         })
///         .await
///         .unwrap();
///
///     Ok(())
/// }
/// ```
///
/// Using with a Kafka consumer loop:
///
/// ```no_run
/// use tokio_stream::StreamExt;
/// use rdkafka::consumer::StreamConsumer;
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     let shutdown = install()?;
///
///     let consumer: StreamConsumer = create_consumer(); // Application-specific.
///     let mut stream = consumer.stream();
///
///     loop {
///         tokio::select! {
///             _ = shutdown.cancelled() => {
///                 break;
///             }
///             message = stream.next() => {
///                 if let Some(Ok(msg)) = message {
///                     // Process message.
///                 }
///             }
///         }
///     }
///
///     Ok(())
/// }
/// ```
pub fn install() -> io::Result<CancellationToken> {
    let token = CancellationToken::new();
    let shutdown_token = token.clone();

    #[cfg(unix)]
    let (mut sigint, mut sigterm) = {
        use tokio::signal::unix::SignalKind;
        (
            tokio::signal::unix::signal(SignalKind::interrupt())?,
            tokio::signal::unix::signal(SignalKind::terminate())?,
        )
    };

    #[cfg(not(unix))]
    let mut sigint = tokio::signal::windows::ctrl_c()?;

    tokio::spawn(async move {
        #[cfg(unix)]
        let terminate = sigterm.recv();

        #[cfg(not(unix))]
        // On non-Unix platforms, a pending future is used as a placeholder.
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = sigint.recv() => {
                info!("received SIGINT (Ctrl+C)");
            }
            _ = terminate => {
                info!("received SIGTERM");
            }
        }

        info!("triggering shutdown");
        shutdown_token.cancel();
    });

    Ok(token)
}
