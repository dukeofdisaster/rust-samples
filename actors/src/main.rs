// covers some async messages using actors
use actix::prelude::*;
use std::thread;
use std::time::Duration;
use rand::prelude::*;

///
/// A mock sensor fn
///
fn read_sensordata() -> f32 {
    random::<f32>() * 10.0
}

#[derive(Debug, Message)]
struct Sensordata(pub u64, pub f32);

// here we implement the actor's message hendling.
// actix requires implementing a Handler<T> trait
struct DBWriter;
impl Actor for DBWriter {
    type Context = SyncContext<Self>;
}
impl Handler<Sensordata> for DBWriter {
    type Result = ();

    fn handle(&mut self, msg: Sensordata, _: &mut Self::Context) -> Self::Result {
        // sned stuff somewhere and handle the results
        println!(" {:?}", msg);
        thread::sleep(Duration::from_millis(300));
    }
}

// in a typical system, an i/o loop would read from sensors in specific 
// intervals, but we forego that here. Instead we implement a for loop.
fn main() -> std::io::Result<()> {
    System::run(|| {
        println!(">> Press Ctrl-C to stop..");
        // start multi threaded actor host (arbiter) with 2 threads
        //let sender = SyncArbiter::start(N_THREADS, || DBWriter);
        let sender = SyncArbiter::start(4, || DBWriter);
        // send messages to the actor
        for n in 0..10_000 { 
            let timestamp = n as u64;
            let data = read_sensordata();
            sender.do_send(Sensordata(timestamp, data));
        }
    })
}

