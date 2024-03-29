use scratch_ctrlc;
use std::sync::mpsc::channel;

fn main() {
    let (tx, rx) = channel();

    scratch_ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");
    println!("Got it! Exiting...");
}
