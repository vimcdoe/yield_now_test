use rtest::*;
fn main() {
    let now = std::time::Instant::now();
    for _ in 0..TOTAL {
        //IO time
        std::thread::yield_now();
    }
    let duration = now.elapsed();
    println!("thread:");
    use_time(TOTAL, duration);
}
