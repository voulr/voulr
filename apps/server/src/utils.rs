use tokio::signal;

pub async fn graceful_shutdown() {
    signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c handler");

    println!("shutting down gracefully...");
}
