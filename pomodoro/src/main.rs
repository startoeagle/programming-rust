use notify_rust as notify;

fn main() {
    notify::Notification::new()
        .summary("Pomodoro")
        .body("Time is up! Focus on task")
        .icon("tomato")
        .show()
        .expect("Could not notify");
}
