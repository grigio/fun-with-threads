// Ported to rust 1.0.0-dev
use std::sync::Arc;
use std::sync::atomic::AtomicUint;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::channel;
use std::thread::Thread;

const NUM_THREADS: usize = 20;
const NUM_INCREMENTS: usize = 10000000us;

fn main() {
  let counter = Arc::new(AtomicUint::new(0));
  let (tx, rx) = channel();
  for _ in range(0us, NUM_THREADS) {
    let (counter, tx) = (counter.clone(), tx.clone());
    Thread::spawn(move || {
      for _ in range(0us, NUM_INCREMENTS) {
        counter.fetch_add(1, Relaxed);
      }
      tx.send(());
    });
  }
  // Wait for threads to finish
  for _ in range(0us, NUM_THREADS) { rx.recv(); }
  println!("{}" , counter.load(Relaxed));
}
