use std::time::{SystemTime};
use std::io;
use std::io::Write;

struct Timer {
    start: SystemTime,
    stop: Option<SystemTime>,
    name: String,
    running: bool,
}

impl Timer {
    fn new(name: String) -> Self {
        Self {
            start: SystemTime::now(),
            stop: None,
            name,
            running: true,
        }
    }
}

fn show_timer(timer: &Timer, name: String) {
    let duration = match timer.stop {
        Some(stop) => stop.duration_since(timer.start).unwrap(),
        None => timer.start.elapsed().unwrap(),
    };
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let seconds = duration.as_secs() % 60;
    let milliseconds = duration.subsec_millis();
    println!("({}) {}: {:02}:{:02}:{:02}.{:03}", (if timer.running { "RUNNING" } else { "STOPPED" }),
             name, hours, minutes, seconds, milliseconds);
}

fn stop_timer(timer: &mut Timer, name: String) {
    timer.stop = Some(SystemTime::now());
    timer.running = false;
    show_timer(timer, name);
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
            ["show", tl @ ..] => {
                if tl.len() == 0 {
                    for timer in &timers {
                        show_timer(timer, timer.name.clone());
                    }
                    continue;
                }
                if tl[0] == "*" {
                    for timer in &timers {
                        show_timer(timer, timer.name.clone());
                    }
                    continue;
                }
                for name in tl {
                    let timer = timers.iter().find(|t| t.name == *name);
                    match timer {
                        Some(t) => show_timer(t, t.name.clone()),
                        None => println!("Timer {} not found", name),
                    }
                }
            },
            ["stop", tl @ ..] => {
                if tl.len() == 0 {
                    println!("No name given");
                    continue;
                }
                if tl[0] == "*" {
                    for timer in &mut timers {
                        stop_timer(timer, timer.name.clone());
                    }
                    continue;
                }
                for name in tl {
                    let timer = timers.iter_mut().find(|t| t.name == *name);
                    match timer {
                        Some(t) => stop_timer(t, t.name.clone()),
                        None => println!("Timer {} not found", name),
                    }
                }
            },
            ["new", tl @ ..] => {
                if tl.len() == 0 {
                    println!("No name given");
                    continue;
                }
                for name in tl {
                    timers.push(Timer::new(name.to_string()));
                }
            },
            [""] => (),
            [m] => println!("Unknown command: {m}"),
            _ => panic!(),
        }
    }
}
