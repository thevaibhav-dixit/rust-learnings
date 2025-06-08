mod async_rust;
mod move_semantics;
mod traits;

#[tokio::main]
async fn main() {
    async_rust::non_blocking().await;
}
