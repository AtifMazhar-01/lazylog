use agent::just_print;
use std::thread::sleep;
use std::time::Duration;
pub fn run_agents() {
    sleep(Duration::from_secs(5));
    just_print();
}
