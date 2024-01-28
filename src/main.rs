// MIT License
//
// Copyright (c) 2024 malloc-nbytes
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


use std::time::SystemTime;
use std::io::Write;
use std::io;

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

fn help() {
    println!("Commands:");
    println!("  help - displays this message");
    println!("  exit - exit the REPL");
    println!("  show <name1> <name2>...|* - show timer(s) `name` or all with `*`");
    println!("  stop <name1> <name2>...|* - stop timer(s) `name` or all with `*`");
    println!("  new <name1> <name2>...|* - create timer(s) `name` or all with `*`");
}

fn main() {
    println!("--- Timer REPL ---");
    println!("Type `help` for more information");

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
            ["help", ..] => help(),
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
