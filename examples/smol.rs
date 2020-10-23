use rtest::*;

fn main() {
    //smol::block_on() 包含io
    smol::future::block_on(async {
        let now = std::time::Instant::now();
        for _ in 0..TOTAL {
            //IO time
            smol::future::yield_now().await;
        }
        let duration = now.elapsed();
        println!("smol:");
        use_time(TOTAL, duration);
    })
}
