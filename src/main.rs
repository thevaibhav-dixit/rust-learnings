mod async_rust;
mod move_semantics;
mod rpn;
mod traits;

#[tokio::main]
async fn main() {
    async_rust::non_blocking().await;
}
