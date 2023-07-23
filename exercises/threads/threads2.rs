// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::os::macos::raw::stat;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for offset in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            // I added `+ (10 * offset)` to play around with concurrency
            thread::sleep(Duration::from_millis(250 + (10 * offset)));
            let mut status_shared = status_shared.lock().unwrap();
            status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
