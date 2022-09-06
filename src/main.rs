#![allow(unused_imports, unused_variables, unused_mut)]

use colored::Colorize;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyCode, KeyEvent, poll};
use crossterm::style::Stylize;
use crossterm::terminal::*;
use crossterm::{event, terminal, ExecutableCommand, QueueableCommand};
use rand::{thread_rng, Rng};
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

type Gen = Vec<Vec<Cell>>;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Alive,
    Dead,
}

fn render_gen(stdout: &mut io::Stdout, gen: &Gen) {
    for i in 0..gen.len() {
        for j in 0..gen[0].len() {
            stdout.queue(MoveTo(i as u16, j as u16)).unwrap();
            match gen[i][j] {
                // Cell::Alive => print!("😎"),
                // Cell::Alive => print!("🦀"),
                Cell::Alive => print!("{}", "X".color("blue")),
                Cell::Dead => print!("{}", "-".color("red")),
            }
            stdout.flush().unwrap();
        }
    }
}

fn new_gen() -> Gen {
    let cells = vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Alive];
    let rows: u16 = terminal::size().unwrap().1;
    let cols: u16 = terminal::size().unwrap().0;
    let mut colums: Vec<Vec<Cell>> = Vec::new();
    for _ in 0..cols {
        let mut col: Vec<Cell> = Vec::new();
        for _ in 0..rows {
            let rand = thread_rng().gen_range(0..10);
            col.push(cells[rand % 5]);
        }
        colums.push(col);
    }
    colums
}

fn is_valid_idx(i: i32, j: i32, m: i32, n: i32) -> bool {
    i >= 0 && i < m && j >= 0 && j < n
}

fn get_alive(x: i32, y: i32, cur_gen: &Gen) -> i32 {
    let mut alive_cnt: i32 = 0;

    let m: i32 = cur_gen.len() as i32;
    let n: i32 = cur_gen[0].len() as i32;
    let dx: [i8; 8] = [0, 1, 0, -1, 1, -1, -1, 1];
    let dy: [i8; 8] = [1, 0, -1, 0, 1, -1, 1, -1];

    for i in 0..8 {
        let nx: i32 = x as i32 + dx[i] as i32;
        let ny: i32 = y as i32 + dy[i] as i32;
        if is_valid_idx(nx, ny, m, n) {
            let cur_cell = cur_gen[nx as usize][ny as usize];
            match cur_cell {
                Cell::Alive => alive_cnt += 1,
                Cell::Dead => (),
            }
        }
    }

    alive_cnt
}

fn next_gen(cur_gen: &mut Gen) -> Gen {
    let m: i32 = cur_gen.len() as i32;
    let n: i32 = cur_gen[0].len() as i32;
    let mut nxt_gen: Gen = Gen::new();
    for _ in 0..m {
        let mut col: Vec<Cell> = Vec::new();
        for _ in 0..n {
            col.push(Cell::Dead);
        }
        nxt_gen.push(col);
    }

    for i in 0..m {
        for j in 0..n {
            let alive = get_alive(i, j, cur_gen);

            match cur_gen[i as usize][j as usize] {
                Cell::Alive => {
                    if alive == 2 || alive == 3 {
                        nxt_gen[i as usize][j as usize] = Cell::Alive;
                    } else {
                        nxt_gen[i as usize][j as usize] = Cell::Dead;
                    }
                }
                Cell::Dead => {
                    if alive == 3 {
                        nxt_gen[i as usize][j as usize] = Cell::Alive;
                    }
                }
            }
        }
    }
    *cur_gen = nxt_gen.clone();
    nxt_gen
}

fn main() {
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
                    KeyCode::Char('a') => 
                        'animate: loop {
                            nxt = next_gen(&mut frame);
                            render_gen(&mut stdout, &nxt);
                            sleep(Duration::from_millis(16));
                            if(poll(Duration::from_millis(1))).unwrap() {
                                if let Event::Key(k) = event::read().unwrap() {
                                    match k.code {
                                        KeyCode::Char('q') => break 'animate,
                                        _ => {}
                                    }
                                }
                            } else {}
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
