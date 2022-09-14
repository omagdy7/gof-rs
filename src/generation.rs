#![allow(unused_imports, unused_variables, unused_mut)]

use crate::ui::*;
use colored::Colorize;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, Event, KeyCode, KeyEvent},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::Stylize,
    terminal,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{thread_rng, Rng};
use std::{
    error::Error,
    io::{self, Write},
    thread::sleep,
    time::Duration,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

pub type Gen = Vec<Vec<Cell>>;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Alive,
    Dead,
}

pub fn render_gen(chunk: &Rect, gen: &Gen) {
    for i in 0..chunk.height as usize {
        for j in 0..chunk.width as usize {
            match gen[i][j] {
                // Cell::Alive => print!("😎"),
                // Cell::Alive => print!("🦀"),
                Cell::Alive => print!("{}", "X".color("blue")),
                Cell::Dead => print!("{}", "-".color("red")),
            }
        }
    }
}

pub fn new_gen(chunk: &Rect, app: &mut App) -> Gen {
    app.flag_cur = true;
    let cells = vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Alive];
    let cols: u16 = chunk.width - 90;
    let rows: u16 = chunk.height - 2;
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    for _ in 0..rows {
        let mut row: Vec<Cell> = Vec::new();
        for _ in 0..cols{
            let rand = thread_rng().gen_range(0..10);
            row.push(cells[rand % 5]);
        }
        grid.push(row);
    }
    grid
}

pub fn gen_to_spans(gen: &Gen) -> Vec<Spans> {
    let mut spans = vec![];
    let alive_cells = vec!["🟥","🟧","🟨","🟩","🟦", "🟪" ,"🟫"];
    for i in 0..gen.len() {
        let mut txt = String::new();
        for j in 0..gen[0].len() {
            let rand = thread_rng().gen_range(0..10);
            match gen[i][j] {
                // Cell::Alive => print!("😎"),
                // Cell::Alive => txt.push_str("🦀"),
                Cell::Alive => txt.push_str(alive_cells[rand % alive_cells.len()]),
                Cell::Dead => txt.push_str("⬛️"),
                // Cell::Alive => txt.push('X'),
                // Cell::Dead => txt.push('-'),
            }
        }
        spans.push(Spans::from(txt));
    }
    spans
}

// pub fn 

pub fn is_valid_idx(i: i32, j: i32, m: i32, n: i32) -> bool {
    i >= 0 && i < m && j >= 0 && j < n
}

pub fn get_alive(x: i32, y: i32, cur_gen: &Gen) -> i32 {
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

pub fn next_gen(app: &mut App) -> Gen {
    let m: i32 = app.cur_gen.len() as i32;
    let n: i32 = app.cur_gen[0].len() as i32;
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
            let alive = get_alive(i, j, &app.cur_gen);

            match app.cur_gen[i as usize][j as usize] {
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
    app.cur_gen = nxt_gen.clone();
    nxt_gen
}

pub fn init() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
