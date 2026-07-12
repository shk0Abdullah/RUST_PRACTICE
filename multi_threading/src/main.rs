// use std::thread;
// fn main() {
// =
//     {
//         let v = vec![1, 2, 3, 4];
//         // move will transfer the ownership to the spawned thread
//         let handle = thread::spawn(move || {
//             println!("this is spawned thread: {v:?} ");
//         });
//         handle.join().unwrap();
//     }

// }

// That's the basic impl of rusts
// Channels for Data Transfering
use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    let v = vec![1, 2, 3, 4];
    let handle = thread::spawn(move || {
        for i in v {
            if i == 3 {
                tx.send(format!("{} is the value i want to send", i))
                    .unwrap();
            }
        }
    });
    handle.join().unwrap();
    let recv = rx.recv().unwrap();
    print!("I am in the main thread {}", recv)
}
// mpsc -> multi producers single consumer :: channel will break into two things transmiter and
// receiver. transmiter sends the data and receiver receives the data
// used to transfer data from threads to threads and here we were transfering the data from one spawned
// thread to the main thread
