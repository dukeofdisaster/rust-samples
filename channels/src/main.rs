use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

// rand has to be added to Cargo.toml
use rand::prelude::*;
use std::time::Duration;

enum ChartValue {
    Star(usize),
    Pipe(usize)
}

fn main() {
    // Multi Producer Single Consumer (mpsc) channels
    // - consists of many senders (with lightweight clone)
    // - under hood channel doesn't lock but uses unsafe struct thata allows
    //  detection and management of the state of the stream. The channel
    //  is just good at sending data across threads and can be used to create
    //  an actor-style framework or a reactive map-reduce style data processor.
    //
    // Fearless concurrency
    //  - data going in is owned by chunnel until rx  becomes owner.
    //  - acts as a queue; frees dev from implementing exchange and adds
    //  concurrency for regular queues for free
    let (tx, rx): (Sender<ChartValue>, Receiver<ChartValue>) = mpsc::channel();

    let pipe_sender = tx.clone();

    // note both of these spawns have infinite loops
    thread::spawn(move || {
        loop {
            pipe_sender.send(ChartValue::Pipe(random::<usize>() %80)).unwrap();
            thread::sleep(Duration::from_millis(random::<u64>() % 800));
        }
    });

    let star_sender = tx.clone();
    thread::spawn(move || {
        loop {
            star_sender.send(ChartValue::Star(random::<usize>() % 80)).unwrap();
            thread::sleep(Duration::from_millis(random::<u64>() % 800));
        }
    });

    // both threads are sending to the channel, so what's missing is therx end
    // to handle the iniput. receiver has two functions, recv() and recv_timeout()
    // both block the calling thread until an item is received or in the latter
    // case, until timeout hits
    while let Ok(val) = rx.recv_timeout(Duration::from_secs(3)) {
        println!("{}", match val {
            ChartValue::Pipe(v) => "|".repeat(v+1),
            ChartValue::Star(v) => "*".repeat(v+1)
        });
    }
}