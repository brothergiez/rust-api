use actix_web::dev::ServerHandle;
use tokio::signal;
use tokio::select;

pub async fn handle_graceful_shutdown(server_handle: ServerHandle) {
    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for shutdown signal");
        println!("Received shutdown signal, stopping server...");
    };

    select! {
        _ = shutdown_signal => {
            server_handle.stop(true).await;
        }
    }

    println!("Server stopped gracefully.");
}
