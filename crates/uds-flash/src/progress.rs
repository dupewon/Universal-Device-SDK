pub trait ProgressReporter {
    fn on_progress(&self, percent: u8, message: &str);
}

pub struct ConsoleProgress;

impl ProgressReporter for ConsoleProgress {
    fn on_progress(&self, percent: u8, message: &str) {
        println!("[{}%] {}", percent, message);
    }
}
