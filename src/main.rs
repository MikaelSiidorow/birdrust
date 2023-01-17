use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use birdrust::{logic::update_report, types::ParsedReport};
use futures::{stream::StreamExt, SinkExt};
use std::{sync::Arc, time::Duration};
use tokio::{sync::broadcast, task, time};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct SharedState {
    // Channel for sending data to clients
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<String>(16);

    let shared_state = Arc::new(SharedState { tx });

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(|| async { "Subscribe to websocket at /reports" }))
        .route(
            "/reports",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |test| websocket_handler(test, shared_state)
            }),
        )
        .layer(cors);

    task::spawn(async move {
        loop {
            // Spawn task for updating the report
            let res = task::spawn(update_task(Arc::clone(&shared_state))).await;
            match res {
                Err(err) if err.is_panic() => println!("Update task panicked, restarting"),
                _ => {
                    println!("Update task failed for unknown reason...");
                    break;
                }
            }
        }
    });

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade, state: Arc<SharedState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<SharedState>) {
    let (mut sender, mut receiver) = stream.split();

    // subscribe to the channel
    let mut rx = state.tx.subscribe();

    // send the initial report
    if let Ok(msg) = rx.recv().await {
        if sender.send(Message::Text(msg)).await.is_err() {
            return;
        }
    }

    // spawn task for sending msgs from the channel to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // spawn task for receiving msgs from the client
    let mut recv_task = tokio::spawn(async move { while let Some(_) = receiver.next().await {} });

    // wait for either task to complete (ie fail)
    tokio::select! {
        _ = &mut send_task => {}
        _ = &mut recv_task => {}
    }

    println!("Client disconnected");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect tokio signal ctrl-c");
    println!("Shutting down");

    // Give the server a second to finish sending responses
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("Shutdown complete");
}

async fn update_task(shared_state: Arc<SharedState>) {
    let mut interval = time::interval(Duration::from_secs(2));
    let mut previous_report: Option<ParsedReport> = None;
    loop {
        interval.tick().await;
        let shared_state = Arc::clone(&shared_state);
        let new_report = update_report(&previous_report).await;
        previous_report = Some(new_report.clone());
        let report_text = serde_json::to_string(&new_report).unwrap();
        match shared_state.tx.send(report_text) {
            Ok(number) => println!("Sent data to {} clients", number),
            Err(_) => println!("No clients connected"),
        };
    }
}
