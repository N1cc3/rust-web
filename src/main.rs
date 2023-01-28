use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;
use warp::ws::{Message, WebSocket};

#[tokio::main]
async fn main() {
	let assets = warp::path("assets").and(warp::fs::dir("front/dist/assets"));
	let index = warp::path::end().and(warp::fs::file("front/dist/index.html"));

	let users = Users::default();
	let users = warp::any().map(move || users.clone());
	let ws = warp::path("ws").and(warp::ws()).and(users).map(|ws: warp::ws::Ws, users| {
		ws.on_upgrade(move |socket| user_connected(socket, users))
	});

	warp::serve(ws.or(assets).or(index)).run(([127, 0, 0, 1], 3030)).await;
}

static NEXT_USER_ID: AtomicI64 = AtomicI64::new(1);

type UserId = i64;
type Users = Arc<RwLock<HashMap<UserId, mpsc::UnboundedSender<Message>>>>;

async fn user_connected(ws: WebSocket, users: Users) {
	let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
	eprintln!("new websocket user: {}", my_id);

	let (mut sender, mut receiver) = ws.split();
	let (sender_stream, receiver_stream) = mpsc::unbounded_channel();
	let mut receiver_stream_unbound = UnboundedReceiverStream::new(receiver_stream);

	tokio::task::spawn(async move {
		while let Some(message) = receiver_stream_unbound.next().await {
			sender
				.send(message)
				.unwrap_or_else(|e| eprintln!("websocket send error: {}", e))
				.await;
		}
	});

	users.write().await.insert(my_id, sender_stream);

	while let Some(result) = receiver.next().await {
		let msg = match result {
			Ok(msg) => msg,
			Err(e) => { eprintln!("websocket error(uid={}): {}", my_id, e); break; }
		};
		user_message(my_id, msg, &users).await;
	}

	user_disconnected(my_id, &users).await;
}

async fn user_message(my_id: UserId, msg: Message, users: &Users) {
	let msg = if let Ok(s) = msg.to_str() { s } else { return; };
	for (&uid, sender) in users.read().await.iter() {
		if my_id != uid {
			if let Err(_disconnected) = sender.send(Message::text(msg.clone())) {
				// May not be needed since already handled in another place
				// user_disconnected(my_id, &users).await;
			}
		}
	}
}

async fn user_disconnected(my_id: UserId, users: &Users) {
	eprintln!("good bye user: {}", my_id);
	users.write().await.remove(&my_id);
}
