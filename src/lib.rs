use std::time::Duration;
pub const TOTAL: i32 = 1_000_000;

//计算每个操作耗时nano纳秒
pub fn use_time(total: i32, duration: Duration) {
    let time_in_ns = duration.as_nanos();
    println!(
        "use Time: {} ns, each: {} nano/op",
        time_in_ns,
        time_in_ns as f64 * 1.0 / (total as f64)
    );
}
