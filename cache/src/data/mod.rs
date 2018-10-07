use std::thread;

pub mod byte_counts;
pub mod prepend;


static CACHE_TICK: u32 = 1 * 60 * 1000;
static mut checkCacheTick: u32 = 0;

fn cacheTicker () {
    loop {
        thread::sleep_ms(CACHE_TICK);
        checkCacheTick += 1;
    }
}
