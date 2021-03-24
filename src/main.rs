use std::process;
use std::time::Duration;
fn main() {
    //ask user for time
    println!("Enter the timer in minutes");
    let mut inp = String::new();
    std::io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read");
    match inp.parse::<u32>() {
        Ok(time) => make_timer(time),
        Err(_) => {
            println!("Cannot parse as u32");
            process::exit(42);
        }
    }
}
fn make_timer(time: u32) {
    let time = time * 60;
    let five = Duration::new(time, 0);
    assert_eq!(five.as_secs(), 5);
}
