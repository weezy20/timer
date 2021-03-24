use std::process;
use std::time::Duration;

fn main() {
    println!("Enter the timer in minutes");
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read");
    match inp.trim().parse::<u32>() {
        Ok(time) => make_timer(time),
        Err(_) => {
            eprintln!("Cannot parse as u32");
            process::exit(42);
        }
    }
}
fn make_timer(time: u32) {
    let time = Duration::new((time * 60).into(), 0);
    use std::thread;
    thread::sleep(time);
    println!("Times Up!");
}
