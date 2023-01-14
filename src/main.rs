use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use birdnest_server::{logic::update_report, types::ParsedReport};
use futures::{stream::StreamExt, SinkExt};
use std::{sync::Arc, time::Duration};
use tokio::{sync::broadcast, task, time};

#[derive(Clone)]
struct SharedState {
    // Channel for sending data to clients
    tx: broadcast::Sender<ParsedReport>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<ParsedReport>(16);

    let shared_state = Arc::new(SharedState { tx });

    let app = Router::new()
        .route("/", get(|| async { "Subscribe to websocket at /reports" }))
        .route(
            "/reports",
            get({
                let shared_state = Arc::clone(&shared_state);
                move |test| websocket_handler(test, shared_state)
            }),
        );

    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(2));
        let mut previous_report: Option<ParsedReport> = None;
        loop {
            interval.tick().await;
            let shared_state = Arc::clone(&shared_state);
            let new_report = update_report(&previous_report).await;
            previous_report = Some(new_report.clone());
            match shared_state.tx.send(new_report) {
                Ok(number) => println!("Sent data to {} clients", number),
                Err(_) => println!("No clients connected"),
            };
        }
    });

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
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

    // spawn task for sending msgs from the channel to the client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender
                .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                .await
                .is_err()
            {
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
