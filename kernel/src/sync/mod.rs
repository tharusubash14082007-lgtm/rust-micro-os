
use spin::Mutex;

pub static SHARED_COUNTER: Mutex<u64> = Mutex::new(0);

pub fn increment_counter() -> u64 {
    let mut counter = SHARED_COUNTER.lock();
    *counter += 1;
    *counter
}

pub fn test_synchronization() -> bool {
    let first = increment_counter();
    let second = increment_counter();

    second == first + 1
}
