use std::collections::{HashMap};
use std::sync::{
	atomic::{AtomicI64, Ordering},
	Arc,
};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

#[tokio::main]
async fn main() {
	// let users = Users::default();
	// let users = warp::any().map(move || users.clone());

	// GET / -> index html
	let index = warp::path("").map(|| "Hello world!");
	warp::serve(index).run(([127, 0, 0, 1], 3030)).await;
}

// static NEXT_USER_ID: AtomicI64 = AtomicI64::new(1);
// type UserId = i64;
// type Users = Arc<RwLock<HashMap<UserId, mpsc::UnboundedSender<Message>>>>;

