use spin_sleep::sleep;
use std::io;
use std::io::Write;
use std::time;

const BELL_CODE: char = '\x07';
const CLEAR_LINE: &'static str = "\x1b[2K";
const CURSOR_TO_LINE_START: char = '\x0d';

fn get_integer() -> u64 {
    loop {
        let mut buffer = String::new();
        if let Err(_) = io::stdin().read_line(&mut buffer) {
            eprintln!("[ERROR] Failed to read from stdin, try again");
            continue;
        };
        match buffer.trim().parse() {
            Ok(value) => return value,
            Err(_) => {
                eprintln!("[ERROR] Failed to parse entry to integer, try again");
                continue;
            }
        }
    }
}

fn main() {
    println!("[INPUT] Enter the number of hours: ");
    let hours = get_integer();
    println!("[INPUT] Enter the number of minutes: ");
    let minutes = get_integer();
    println!("[INPUT] Enter the number of seconds: ");
    let seconds = get_integer();
    let total_seconds = minutes * 60 + hours * 3600 + seconds;
    println!("[INFO] Timer started");
    let start = time::Instant::now();
    println!("[TIMER] Remaining time: ");
    while start.elapsed().as_secs() < total_seconds {
        let remaining_seconds = total_seconds - start.elapsed().as_secs();
        let percentage_left = remaining_seconds as f32 * 100. / total_seconds as f32;
        print!("{CLEAR_LINE}{CURSOR_TO_LINE_START}");
        print!(
            "{:02}:{:02}:{:02} [{:.1}% Time Left]",
            remaining_seconds / 3600,
            (remaining_seconds / 60) % 60,
            remaining_seconds % 60,
            percentage_left
        );
        io::stdout().flush().unwrap();
        sleep(time::Duration::new(1, 0));
    }
    println!();
    println!("[INFO] Timer Expired {BELL_CODE}");
}
