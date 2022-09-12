use crate::generation::*;

use colored::Colorize;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event,
    event::{poll, Event, KeyCode, KeyEvent},
    style::Stylize,
    terminal,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use rand::{thread_rng, Rng};
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};


pub fn run() {
    let mut stdout = io::stdout();
    let mut frame: Gen = new_gen();
    let mut nxt;

    terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(Hide).unwrap();

    'gameloop: loop {
        while event::poll(Duration::default()).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Char('s') => {
                        frame = new_gen();
                        render_gen(&mut stdout, &frame)
                    }

                    KeyCode::Char('n') => {
                        nxt = next_gen(&mut frame);
                        render_gen(&mut stdout, &nxt)
                    }
                    KeyCode::Char('a') => 'animate: loop {
                        nxt = next_gen(&mut frame);
                        render_gen(&mut stdout, &nxt);
                        sleep(Duration::from_millis(16));
                        if (poll(Duration::from_millis(1))).unwrap() {
                            if let Event::Key(k) = event::read().unwrap() {
                                match k.code {
                                    KeyCode::Char('q') => break 'animate,
                                    _ => {}
                                }
                            }
                        } else {
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    stdout.execute(Show).unwrap();
    stdout.execute(LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();

}
