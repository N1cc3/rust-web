use warp::Filter;

#[tokio::main]
async fn main() {
	// let users = Users::default();
	// let users = warp::any().map(move || users.clone());

	let static_dir = warp::path("assets").and(warp::fs::dir("front/dist/assets"));
	let index = warp::path::end().and(warp::fs::file("front/dist/index.html"));
	warp::serve(static_dir.or(index)).run(([127, 0, 0, 1], 3030)).await;
}

// static NEXT_USER_ID: AtomicI64 = AtomicI64::new(1);
// type UserId = i64;
// type Users = Arc<RwLock<HashMap<UserId, mpsc::UnboundedSender<Message>>>>;

