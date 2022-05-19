use rand::random;
use std::sync::mpsc;
use std::thread;

fn generate() -> Vec<u8> {
    thread::sleep(std::time::Duration::from_secs(1));
    (0..1000).into_iter().map(|_| random()).collect()
}

fn main() {
    let (sender, reciever) = mpsc::channel();
    thread::spawn(move || {
        let numframes = 100;
        for _ in 0..numframes {
            sender.send(generate()).unwrap()
        }
    });

    for data in reciever {
        let mean = data
            .iter()
            .fold(0.0, |sum, x| (sum + *x as f32) / data.len() as f32);
        println!("The mean is {}", mean);
    }
}
