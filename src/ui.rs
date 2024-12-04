use crate::generation::*;
use crossterm::event::{self, Event, KeyCode};
use include_dir::{include_dir, Dir};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use std::{io, thread::sleep, time::Duration};

// for including the patterns dir in compile time
static PRESETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/presets/patterns/");

// struct that holds all the information in the left side bar
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
pub struct App {
    pub items: StatefulList<(String, String)>,
    pub flag_cur: bool,
    pub layout: Layout,
    pub cur_gen: Gen,
}

impl App {
    pub fn new() -> App {
        // read the patterns from the files and load it in a vector
        fn read_presets() -> Vec<(String, String)> {
            let mut result = Vec::new();
            for i in 1..=513 {
                let file_name = format!("pattern{}.txt", i);
                let file = PRESETS_DIR.get_file(file_name.to_owned()).unwrap();
                let body = file.contents_utf8().unwrap().to_owned();
                result.push((file_name, body));
            }
            result
        }
        App {
            items: StatefulList::with_items(read_presets()),
            flag_cur: false,
            layout: Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(12), Constraint::Percentage(90)].as_ref()),
            cur_gen: Gen::new(),
        }
    }
}

// gameloop and event handling
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut item_cnt = 1;
    loop {
        terminal.draw(|f| ui_list(f, &mut app))?;
        let timeout = Duration::from_millis(32);
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Left | KeyCode::Char('h') => app.items.unselect(),
                    KeyCode::Down | KeyCode::Char('j') => {
                        if item_cnt >= app.items.items.len() - 1 {
                            item_cnt = 0;
                        } else {
                            item_cnt += 1;
                        }
                        app.items.next();
                        app.cur_gen = gen_from_file(&app.items.items[item_cnt].1);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if item_cnt == 0 {
                            item_cnt = app.items.items.len() - 1;
                        } else {
                            item_cnt -= 1;
                        }
                        app.items.previous();
                        app.cur_gen = gen_from_file(&app.items.items[item_cnt].1);
                    }
                    KeyCode::Char('n') => {
                        terminal.draw(|f| ui_game(f, &mut app))?;
                    }
                    KeyCode::Char('a') => 'animate: loop {
                        app.layout = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(
                                [Constraint::Percentage(0), Constraint::Percentage(100)].as_ref(),
                            );
                        terminal.draw(|f| ui_game(f, &mut app))?;
                        sleep(Duration::from_millis(32));
                        if (crossterm::event::poll(Duration::from_millis(1))).unwrap() {
                            if let Event::Key(k) = event::read().unwrap() {
                                match k.code {
                                    KeyCode::Char('s') => {
                                        break 'animate app.layout = Layout::default()
                                            .direction(Direction::Horizontal)
                                            .constraints(
                                                [
                                                    Constraint::Percentage(12),
                                                    Constraint::Percentage(90),
                                                ]
                                                .as_ref(),
                                            );
                                    }
                                    _ => {}
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn ui_list(f: &mut Frame, app: &mut App) {
    let chunks = app.layout.split(f.area());

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.0.as_str())];
            ListItem::new(lines).style(Style::default().fg(Color::Red).bg(Color::Reset))
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Cool Paterns"))
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .add_modifier(Modifier::ITALIC),
        )
        .highlight_symbol("-> ");

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    if !app.flag_cur {
        app.cur_gen = new_gen(app);
    }
    let spans = gen_to_spans(&app.cur_gen);
    render_gen(f, chunks[1], &spans);
}

fn ui_game(f: &mut Frame, app: &mut App) {
    let chunks = app.layout.split(f.area());

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let lines = vec![Line::from(i.0.as_str())];
            ListItem::new(lines).style(Style::default().fg(Color::Red).bg(Color::Black))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cool Patterns"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    let nxt = next_gen(app);
    let spans = gen_to_spans(&nxt);
    render_gen(f, chunks[1], &spans)
}
