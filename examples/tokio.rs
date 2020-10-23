use rtest::*;
#[tokio::main]
async fn main() {
    let now = std::time::Instant::now();
    for _ in 0..TOTAL {
        //IO time
        tokio::task::yield_now().await;
    }
    let duration = now.elapsed();
    println!("tokio:");
    use_time(TOTAL, duration);
}
