use crate::ui::*;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{thread_rng, Rng};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

pub type Gen = Vec<Vec<Cell>>;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Alive,
    Dead,
}

pub fn render_gen<B: Backend>(f: &mut Frame<B>, chunk: Rect, spans: &Vec<Spans>) {
    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::Red))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center)
    };
    let paragraph = Paragraph::new(spans.clone())
        .style(Style::default().bg(Color::Black).fg(Color::Blue))
        .block(create_block(" Conway's Game-Of-Life "))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunk);
}

pub fn new_gen(app: &mut App) -> Gen {
    app.flag_cur = true;
    let cells = vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Alive];
    let cols: u16 = 72;
    let rows: u16 = 44;
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    for _ in 0..rows {
        let mut row: Vec<Cell> = Vec::new();
        for _ in 0..cols {
            let rand = thread_rng().gen_range(0..10);
            row.push(cells[rand % 5]);
        }
        grid.push(row);
    }
    grid
}

pub fn gen_to_spans(gen: &Gen) -> Vec<Spans> {
    let mut spans = vec![];
    // let alive_cells = vec!["🟥", "🟦", "🟨", "🟪", "🟧", "🟩", "🟫"];
    for i in 0..gen.len() {
        let mut txt = String::new();
        for j in 0..gen[0].len() {
            match gen[i][j] {
                // Cell::Alive => txt.push_str(alive_cells[rand % alive_cells.len()]),
                Cell::Alive => txt.push_str("⬜"),
                // Cell::Dead => txt.push_str("⬛️"),
                Cell::Dead => txt.push_str("  "),
                // Cell::Alive => print!("😎"),
                // Cell::Alive => txt.push_str("🦀"),
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

pub fn gen_from_file(s: &String) -> Gen {
    let mut gen = Gen::new();
    for line in s.lines() {
        let line = line;
        let mut row: Vec<Cell> = vec![];
        for ch in line.chars() {
            if ch == '.' {
                row.push(Cell::Dead);
            } else {
                row.push(Cell::Alive);
            }
        }
        gen.push(row);
    }
    gen
}

pub fn init() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

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
