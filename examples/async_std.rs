use rtest::*;

#[async_std::main]
async fn main() {
    let now = std::time::Instant::now();
    for _ in 0..TOTAL {
        //IO time
        async_std::task::yield_now().await;
    }
    let duration = now.elapsed();
    println!("async_std:");
    use_time(TOTAL, duration);
}
