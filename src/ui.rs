use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    thread::sleep,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::generation::*;

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
pub struct App<'a> {
    pub items: StatefulList<(&'a str, usize)>,
    pub flag: bool,
    pub layout: Layout,
    pub cur_gen: Gen,
    pub nxt_gen: Gen,
    // generation: Gen,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Glider", 1),
                ("Glider", 2),
                ("Glider", 1),
                ("Glider", 3),
            ]),
            flag: false,
            layout: Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref()),
            cur_gen: Gen::new(),
            nxt_gen: Gen::new(),
        }
    }

    // Rotate through the event list.
    // This only exists to simulate some kind of "progress"
    // fn on_tick(&mut self) {
    //     let event = self.events.remove(0);
    //     self.events.push(event);
    // }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        // let mut frame = app.generation;
        // let nxt;

        // let timeout = tick_rate
        //     .checked_sub(last_tick.elapsed())
        //     .unwrap_or_else(|| Duration::from_secs(0));
        let timeout = Duration::from_millis(40);
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left | KeyCode::Char('h') => app.items.unselect(),
                    KeyCode::Down | KeyCode::Char('j') => app.items.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.items.previous(),
                    _ => {} // KeyCode::Char('s') => {
                            //     frame = new_gen();
                            //     render_gen(&mut stdout, &frame);
                            //     Ok(());
                            // }
                            // KeyCode::Char('n') => {
                            //     nxt = next_gen(&mut frame);
                            //     render_gen(&mut stdout, &nxt)
                            // }
                            // KeyCode::Char('a') => 'animate: loop {
                            //     nxt = next_gen(&mut frame);
                            //     render_gen(&mut stdout, &nxt);
                            //     sleep(Duration::from_millis(16));
                            //     if (crossterm::event::poll(Duration::from_millis(1))).unwrap() {
                            //         if let Event::Key(k) = event::read().unwrap() {
                            //             match k.code {
                            //                 KeyCode::Char('q') => break 'animate,
                            //                 _ => {}
                            //             }
                            //         }
                            //     } else {
                            //     }
                            // },
                            // _ => {}
                }
            }
        }
        // if last_tick.elapsed() >= tick_rate {
        //     app.on_tick();
        //     last_tick = Instant::now();
        // }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    // let chunks = Layout::default()
    //     .direction(Direction::Horizontal)
    //     .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref());
    // .split(f.size());

    let chunks = app.layout.split(f.size());
    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.0)];
            ListItem::new(lines).style(Style::default().fg(Color::Red).bg(Color::Black))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    if !app.flag {
        app.cur_gen = new_gen(&chunks[1]);
    }
    let nxt = next_gen(app);
    let spans = gen_to_spans(&nxt);

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
        .block(create_block(" Game Of Life "))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[1]);
}
