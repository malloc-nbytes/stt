use std::time::{SystemTime};
use std::io;
use std::io::Write;

struct Timer {
    start: SystemTime,
    stop: Option<SystemTime>,
    name: String,
}

impl Timer {
    fn new(name: String) -> Self {
        Self {
            start: SystemTime::now(),
            stop: None,
            name
        }
    }
}

fn add_timer(timers: &mut Vec<Timer>, name: String) {
    if timers.iter().any(|t| t.name == name) {
        println!("[ERR]: Timer: {} already exists.", name);
    } else {
        timers.push(Timer::new(name));
    }
}

fn stop_timer(timers: &mut Vec<Timer>, name: String) {
    if let Some(timer) = timers.iter_mut().find(|t| t.name == name) {
        timer.stop = Some(SystemTime::now());
    } else {
        println!("[ERR]: Timer {name} does not exist.");
    }
}

fn show_timer(timers: &mut Vec<Timer>, name: String) {
    if let Some(timer) = timers.iter().find(|t| t.name == name) {
        if let Some(stop) = timer.stop {
            println!("Timer {name} ran for {seconds} seconds.", seconds = stop.duration_since(timer.start).unwrap().as_secs());
        } else {
            println!("Timer {name} is still running.");
        }
    } else {
        println!("[ERR]: Timer {name} does not exist.");
    }
}

fn main() {
    let mut timers: Vec<Timer> = Vec::new();
    let mut inp = String::new();

    loop {
        inp.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read input");

        let parts: Vec<&str> = inp.trim().split(" ").collect();

        match &parts[..] {
            ["exit", ..] => break,
            ["show", tl @ ..] => todo!(),
            ["stop", tl @ ..] => todo!(),
            ["new", tl @ ..] => todo!(),
            [""] => (),
            [m] => println!("Unknown command: {m}"),
            _ => panic!(),
        }
    }
}
