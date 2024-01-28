use std::time::{SystemTime};
use std::io;
use std::io::Write;

struct Timer {
    start: SystemTime,
    stop: Option<SystemTime>,
    name: String,
    running: bool,
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
    if timer.stop.is_some() {
        println!("{name} is already stopped");
        return;
    }
    timer.stop = Some(SystemTime::now());
    timer.running = false;
    show_timer(timer, name);
}

fn show(timers: &Vec<Timer>, tl: &[&str]) {
    if tl.len() == 0 {
        for timer in timers {
            show_timer(timer, timer.name.clone());
        }
        return;
    }
    if tl[0] == "*" {
        for timer in timers {
            show_timer(timer, timer.name.clone());
        }
        return;
    }
    for name in tl {
        let timer = timers.iter().find(|t| t.name == *name);
        match timer {
            Some(t) => show_timer(t, t.name.clone()),
            None => println!("Timer {} not found", name),
        }
    }
}

fn create(timers: &mut Vec<Timer>, tl: &[&str]) {
    if tl.len() == 0 {
        println!("No name given");
        return;
    }
    for name in tl {
        timers.push(Timer {
            start: SystemTime::now(),
            stop: None,
            name: name.to_string(),
            running: true,
        });
    }
}

fn stop(timers: &mut Vec<Timer>, tl: &[&str]) {
    if tl.len() == 0 {
        println!("No name given");
        return;
    }
    if tl[0] == "*" {
        for timer in timers {
            stop_timer(timer, timer.name.clone());
        }
        return;
    }
    for name in tl {
        let timer = timers.iter_mut().find(|t| t.name == *name);
        match timer {
            Some(t) => stop_timer(t, t.name.clone()),
            None => println!("Timer {} not found", name),
        }
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
            ["show", tl @ ..] => show(&timers, &*tl),
            ["stop", tl @ ..] => stop(&mut timers, &*tl),
            ["new", tl @ ..] => create(&mut timers, &*tl),
            [""] => (),
            [m] => println!("Unknown command: {m}"),
            _ => panic!(),
        }
    }
}
