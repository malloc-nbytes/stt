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

fn add_timers(timers: &Timer, names: Vec<String>) {
    unimplemented!()
}

fn stop_timers(timers: &Timer, names: Vec<String>) {
    unimplemented!()
}

fn list_timers(timers: &Timer, names: Vec<String>) {
    unimplemented!()
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
            ["ls", tl @ ..] => todo!(),
            [""] => (),
            [m] => println!("Unknown command: {m}"),
            _ => panic!(),
        }
    }
}
