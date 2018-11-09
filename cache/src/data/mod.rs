use std::thread;

pub mod byte_counts;


static CACHE_TICK: u32 = 1 * 60 * 1000;
static mut CHECK_CACHE_TICK: u32 = 0;

fn cache_cicker() {
    loop {
        thread::sleep_ms(CACHE_TICK);
        unsafe {
            CHECK_CACHE_TICK += 1;
        }
    }
}
