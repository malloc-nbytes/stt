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

use crossterm::{
    execute,
    style::{Print, ResetColor, SetForegroundColor, Color},
    terminal::{Clear, ClearType},
    cursor::{MoveTo},
};

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

    let (h, m, s, ms) = (
        duration.as_secs() / 3600,
        (duration.as_secs() % 3600) / 60,
        duration.as_secs() % 60,
        duration.subsec_millis(),
    );

    let status_color = if timer.running {Color::Green} else {Color::Red};

    execute!(
        std::io::stdout(),
        SetForegroundColor(status_color),
        Print(format!(
            "({}) {}: {:02}:{:02}:{:02}.{:03}",
            if timer.running { "RUNNING" } else { "STOPPED" },
            name,
            h,
            m,
            s,
            ms
        )),
        ResetColor
    )
    .expect("Failed to set colors");

    println!();
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
        if timers.iter().any(|t| t.name == *name.to_string()) {
            println!("timer {name} is already created");
        } else {
            timers.push(Timer {
                start: SystemTime::now(),
                stop: None,
                name: name.to_string(),
                running: true,
            });
        }
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

fn clear() {
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print("\n"),
        ResetColor
    ).expect("Failed to clear the terminal")
}

fn help() {
    println!("Commands:");
    println!("  help|h  - displays this message");
    println!("  exit|e  - exit the REPL");
    println!("  show|sh - <name1> <name2>...|* - show timer(s) `name` or all with `*`");
    println!("  stop|st - <name1> <name2>...|* - stop timer(s) `name` or all with `*`");
    println!("  new|n   - <name1> <name2>...|* - create timer(s) `name` or all with `*`");
    println!("  clear|c - clear the terminal");
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
            ["help" | "h", ..] => help(),
            ["exit" | "e", ..] => break,
            ["show" | "sh", tl @ ..] => show(&timers, &*tl),
            ["stop" | "st", tl @ ..] => stop(&mut timers, &*tl),
            ["new" | "n", tl @ ..] => create(&mut timers, &*tl),
            ["clear" | "c", ..] => clear(),
            [""] => (),
            [m] => println!("Unknown command: {m}"),
            _ => panic!(),
        }
    }
}
