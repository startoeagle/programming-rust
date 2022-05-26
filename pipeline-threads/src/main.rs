use std::sync::mpsc;
use std::thread;

trait OffThreadExt: Iterator {
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
where
    T: Iterator + Send + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> mpsc::IntoIter<T::Item> {
        let (sender, receiver) = mpsc::sync_channel(1024);
        thread::spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });

        receiver.into_iter()
    }
}

fn main() {
    let lines = vec!["anton", "karlsson"];
    let res: usize = lines
        .into_iter()
        .off_thread() // creates a new thread
        .map(|line| {
            line.chars()
                .filter(|c| "aeiou".chars().any(|v| v == *c))
                .collect::<String>()
        })
        .off_thread()
        .map(|line| line.len())
        .off_thread()
        .sum();

    println!("The results is {}", res);
}
