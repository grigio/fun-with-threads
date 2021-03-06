use std::sync::{Arc, Mutex};
const NUM_THREADS: uint = 20;
const NUM_INCREMENTS: uint = 1000000u;

fn main() {
    let data = Arc::new(Mutex::new(0u));
    let (tx, rx) = channel();
    for _ in range(0u, NUM_THREADS) {
        let (data, tx) = (data.clone(), tx.clone());
        spawn(proc() {
            for _ in range(0u, NUM_INCREMENTS) {
                let mut d = data.lock();
                *d += 1;
            }
            tx.send(());
        })
    }
    // Wait for threads to finish
    for _ in range(0u, NUM_THREADS) { rx.recv(); }
    let d = data.lock();
    println!("{}" , *d);
}
