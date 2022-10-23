use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    path::Path,
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
    pub items: StatefulList<(&'a str, &'a Path)>,
    pub flag_cur: bool,
    pub layout: Layout,
    pub cur_gen: Gen,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                (
                    "pattern1",
                    Path::new("./presets/patterns/pattern1.txt")
                ),
                (
                    "pattern2",
                    Path::new("./presets/patterns/pattern2.txt")
                ),

                (
                    "pattern3",
                    Path::new("./presets/patterns/pattern3.txt")
                ),

                (
                    "pattern4",
                    Path::new("./presets/patterns/pattern4.txt")
                ),

                (
                    "pattern5",
                    Path::new("./presets/patterns/pattern5.txt")
                ),

                (
                    "pattern6",
                    Path::new("./presets/patterns/pattern6.txt")
                ),

                (
                    "pattern7",
                    Path::new("./presets/patterns/pattern7.txt")
                ),

                (
                    "pattern8",
                    Path::new("./presets/patterns/pattern8.txt")
                ),

                (
                    "pattern9",
                    Path::new("./presets/patterns/pattern9.txt")
                ),

                (
                    "pattern10",
                    Path::new("./presets/patterns/pattern10.txt")
                ),

                (
                    "pattern11",
                    Path::new("./presets/patterns/pattern11.txt")
                ),

                (
                    "pattern12",
                    Path::new("./presets/patterns/pattern12.txt")
                ),

                (
                    "pattern13",
                    Path::new("./presets/patterns/pattern13.txt")
                ),

                (
                    "pattern14",
                    Path::new("./presets/patterns/pattern14.txt")
                ),

                (
                    "pattern15",
                    Path::new("./presets/patterns/pattern15.txt")
                ),

                (
                    "pattern16",
                    Path::new("./presets/patterns/pattern16.txt")
                ),

                (
                    "pattern17",
                    Path::new("./presets/patterns/pattern17.txt")
                ),

                (
                    "pattern18",
                    Path::new("./presets/patterns/pattern18.txt")
                ),

                (
                    "pattern19",
                    Path::new("./presets/patterns/pattern19.txt")
                ),

                (
                    "pattern20",
                    Path::new("./presets/patterns/pattern20.txt")
                ),

                (
                    "pattern21",
                    Path::new("./presets/patterns/pattern21.txt")
                ),

                (
                    "pattern22",
                    Path::new("./presets/patterns/pattern22.txt")
                ),

                (
                    "pattern23",
                    Path::new("./presets/patterns/pattern23.txt")
                ),

                (
                    "pattern24",
                    Path::new("./presets/patterns/pattern24.txt")
                ),

                (
                    "pattern25",
                    Path::new("./presets/patterns/pattern25.txt")
                ),

                (
                    "pattern26",
                    Path::new("./presets/patterns/pattern26.txt")
                ),

                (
                    "pattern27",
                    Path::new("./presets/patterns/pattern27.txt")
                ),

                (
                    "pattern28",
                    Path::new("./presets/patterns/pattern28.txt")
                ),

                (
                    "pattern29",
                    Path::new("./presets/patterns/pattern29.txt")
                ),

                (
                    "pattern30",
                    Path::new("./presets/patterns/pattern30.txt")
                ),

                (
                    "pattern31",
                    Path::new("./presets/patterns/pattern31.txt")
                ),

                (
                    "pattern32",
                    Path::new("./presets/patterns/pattern32.txt")
                ),

                (
                    "pattern33",
                    Path::new("./presets/patterns/pattern33.txt")
                ),

                (
                    "pattern34",
                    Path::new("./presets/patterns/pattern34.txt")
                ),

                (
                    "pattern35",
                    Path::new("./presets/patterns/pattern35.txt")
                ),

                (
                    "pattern36",
                    Path::new("./presets/patterns/pattern36.txt")
                ),

                (
                    "pattern37",
                    Path::new("./presets/patterns/pattern37.txt")
                ),

                (
                    "pattern38",
                    Path::new("./presets/patterns/pattern38.txt")
                ),

                (
                    "pattern39",
                    Path::new("./presets/patterns/pattern39.txt")
                ),

                (
                    "pattern40",
                    Path::new("./presets/patterns/pattern40.txt")
                ),

                (
                    "pattern41",
                    Path::new("./presets/patterns/pattern41.txt")
                ),

                (
                    "pattern42",
                    Path::new("./presets/patterns/pattern42.txt")
                ),

                (
                    "pattern43",
                    Path::new("./presets/patterns/pattern43.txt")
                ),

                (
                    "pattern44",
                    Path::new("./presets/patterns/pattern44.txt")
                ),

                (
                    "pattern45",
                    Path::new("./presets/patterns/pattern45.txt")
                ),

                (
                    "pattern46",
                    Path::new("./presets/patterns/pattern46.txt")
                ),

                (
                    "pattern47",
                    Path::new("./presets/patterns/pattern47.txt")
                ),

                (
                    "pattern48",
                    Path::new("./presets/patterns/pattern48.txt")
                ),

                (
                    "pattern49",
                    Path::new("./presets/patterns/pattern49.txt")
                ),

                (
                    "pattern50",
                    Path::new("./presets/patterns/pattern50.txt")
                ),

                (
                    "pattern51",
                    Path::new("./presets/patterns/pattern51.txt")
                ),

                (
                    "pattern52",
                    Path::new("./presets/patterns/pattern52.txt")
                ),

                (
                    "pattern53",
                    Path::new("./presets/patterns/pattern53.txt")
                ),

                (
                    "pattern54",
                    Path::new("./presets/patterns/pattern54.txt")
                ),

                (
                    "pattern55",
                    Path::new("./presets/patterns/pattern55.txt")
                ),

                (
                    "pattern56",
                    Path::new("./presets/patterns/pattern56.txt")
                ),

                (
                    "pattern57",
                    Path::new("./presets/patterns/pattern57.txt")
                ),

                (
                    "pattern58",
                    Path::new("./presets/patterns/pattern58.txt")
                ),

                (
                    "pattern59",
                    Path::new("./presets/patterns/pattern59.txt")
                ),

                (
                    "pattern60",
                    Path::new("./presets/patterns/pattern60.txt")
                ),

                (
                    "pattern61",
                    Path::new("./presets/patterns/pattern61.txt")
                ),

                (
                    "pattern62",
                    Path::new("./presets/patterns/pattern62.txt")
                ),

                (
                    "pattern63",
                    Path::new("./presets/patterns/pattern63.txt")
                ),

                (
                    "pattern64",
                    Path::new("./presets/patterns/pattern64.txt")
                ),

                (
                    "pattern65",
                    Path::new("./presets/patterns/pattern65.txt")
                ),

                (
                    "pattern66",
                    Path::new("./presets/patterns/pattern66.txt")
                ),

                (
                    "pattern67",
                    Path::new("./presets/patterns/pattern67.txt")
                ),

                (
                    "pattern68",
                    Path::new("./presets/patterns/pattern68.txt")
                ),

                (
                    "pattern69",
                    Path::new("./presets/patterns/pattern69.txt")
                ),

                (
                    "pattern70",
                    Path::new("./presets/patterns/pattern70.txt")
                ),

                (
                    "pattern71",
                    Path::new("./presets/patterns/pattern71.txt")
                ),

                (
                    "pattern72",
                    Path::new("./presets/patterns/pattern72.txt")
                ),

                (
                    "pattern73",
                    Path::new("./presets/patterns/pattern73.txt")
                ),

                (
                    "pattern74",
                    Path::new("./presets/patterns/pattern74.txt")
                ),

                (
                    "pattern75",
                    Path::new("./presets/patterns/pattern75.txt")
                ),

                (
                    "pattern76",
                    Path::new("./presets/patterns/pattern76.txt")
                ),

                (
                    "pattern77",
                    Path::new("./presets/patterns/pattern77.txt")
                ),

                (
                    "pattern78",
                    Path::new("./presets/patterns/pattern78.txt")
                ),

                (
                    "pattern79",
                    Path::new("./presets/patterns/pattern79.txt")
                ),

                (
                    "pattern80",
                    Path::new("./presets/patterns/pattern80.txt")
                ),

                (
                    "pattern81",
                    Path::new("./presets/patterns/pattern81.txt")
                ),

                (
                    "pattern82",
                    Path::new("./presets/patterns/pattern82.txt")
                ),

                (
                    "pattern83",
                    Path::new("./presets/patterns/pattern83.txt")
                ),

                (
                    "pattern84",
                    Path::new("./presets/patterns/pattern84.txt")
                ),

                (
                    "pattern85",
                    Path::new("./presets/patterns/pattern85.txt")
                ),

                (
                    "pattern86",
                    Path::new("./presets/patterns/pattern86.txt")
                ),

                (
                    "pattern87",
                    Path::new("./presets/patterns/pattern87.txt")
                ),

                (
                    "pattern88",
                    Path::new("./presets/patterns/pattern88.txt")
                ),

                (
                    "pattern89",
                    Path::new("./presets/patterns/pattern89.txt")
                ),

                (
                    "pattern90",
                    Path::new("./presets/patterns/pattern90.txt")
                ),

                (
                    "pattern91",
                    Path::new("./presets/patterns/pattern91.txt")
                ),

                (
                    "pattern92",
                    Path::new("./presets/patterns/pattern92.txt")
                ),

                (
                    "pattern93",
                    Path::new("./presets/patterns/pattern93.txt")
                ),

                (
                    "pattern94",
                    Path::new("./presets/patterns/pattern94.txt")
                ),

                (
                    "pattern95",
                    Path::new("./presets/patterns/pattern95.txt")
                ),

                (
                    "pattern96",
                    Path::new("./presets/patterns/pattern96.txt")
                ),

                (
                    "pattern97",
                    Path::new("./presets/patterns/pattern97.txt")
                ),

                (
                    "pattern98",
                    Path::new("./presets/patterns/pattern98.txt")
                ),

                (
                    "pattern99",
                    Path::new("./presets/patterns/pattern99.txt")
                ),

                (
                    "pattern100",
                    Path::new("./presets/patterns/pattern100.txt")
                ),

                (
                    "pattern101",
                    Path::new("./presets/patterns/pattern101.txt")
                ),

                (
                    "pattern102",
                    Path::new("./presets/patterns/pattern102.txt")
                ),

                (
                    "pattern103",
                    Path::new("./presets/patterns/pattern103.txt")
                ),

                (
                    "pattern104",
                    Path::new("./presets/patterns/pattern104.txt")
                ),

                (
                    "pattern105",
                    Path::new("./presets/patterns/pattern105.txt")
                ),

                (
                    "pattern106",
                    Path::new("./presets/patterns/pattern106.txt")
                ),

                (
                    "pattern107",
                    Path::new("./presets/patterns/pattern107.txt")
                ),

                (
                    "pattern108",
                    Path::new("./presets/patterns/pattern108.txt")
                ),

                (
                    "pattern109",
                    Path::new("./presets/patterns/pattern109.txt")
                ),

                (
                    "pattern110",
                    Path::new("./presets/patterns/pattern110.txt")
                ),

                (
                    "pattern111",
                    Path::new("./presets/patterns/pattern111.txt")
                ),

                (
                    "pattern112",
                    Path::new("./presets/patterns/pattern112.txt")
                ),

                (
                    "pattern113",
                    Path::new("./presets/patterns/pattern113.txt")
                ),

                (
                    "pattern114",
                    Path::new("./presets/patterns/pattern114.txt")
                ),

                (
                    "pattern115",
                    Path::new("./presets/patterns/pattern115.txt")
                ),

                (
                    "pattern116",
                    Path::new("./presets/patterns/pattern116.txt")
                ),

                (
                    "pattern117",
                    Path::new("./presets/patterns/pattern117.txt")
                ),

                (
                    "pattern118",
                    Path::new("./presets/patterns/pattern118.txt")
                ),

                (
                    "pattern119",
                    Path::new("./presets/patterns/pattern119.txt")
                ),

                (
                    "pattern120",
                    Path::new("./presets/patterns/pattern120.txt")
                ),

                (
                    "pattern121",
                    Path::new("./presets/patterns/pattern121.txt")
                ),

                (
                    "pattern122",
                    Path::new("./presets/patterns/pattern122.txt")
                ),

                (
                    "pattern123",
                    Path::new("./presets/patterns/pattern123.txt")
                ),

                (
                    "pattern124",
                    Path::new("./presets/patterns/pattern124.txt")
                ),

                (
                    "pattern125",
                    Path::new("./presets/patterns/pattern125.txt")
                ),

                (
                    "pattern126",
                    Path::new("./presets/patterns/pattern126.txt")
                ),

                (
                    "pattern127",
                    Path::new("./presets/patterns/pattern127.txt")
                ),

                (
                    "pattern128",
                    Path::new("./presets/patterns/pattern128.txt")
                ),

                (
                    "pattern129",
                    Path::new("./presets/patterns/pattern129.txt")
                ),

                (
                    "pattern130",
                    Path::new("./presets/patterns/pattern130.txt")
                ),

                (
                    "pattern131",
                    Path::new("./presets/patterns/pattern131.txt")
                ),

                (
                    "pattern132",
                    Path::new("./presets/patterns/pattern132.txt")
                ),

                (
                    "pattern133",
                    Path::new("./presets/patterns/pattern133.txt")
                ),

                (
                    "pattern134",
                    Path::new("./presets/patterns/pattern134.txt")
                ),

                (
                    "pattern135",
                    Path::new("./presets/patterns/pattern135.txt")
                ),

                (
                    "pattern136",
                    Path::new("./presets/patterns/pattern136.txt")
                ),

                (
                    "pattern137",
                    Path::new("./presets/patterns/pattern137.txt")
                ),

                (
                    "pattern138",
                    Path::new("./presets/patterns/pattern138.txt")
                ),

                (
                    "pattern139",
                    Path::new("./presets/patterns/pattern139.txt")
                ),

                (
                    "pattern140",
                    Path::new("./presets/patterns/pattern140.txt")
                ),

                (
                    "pattern141",
                    Path::new("./presets/patterns/pattern141.txt")
                ),

                (
                    "pattern142",
                    Path::new("./presets/patterns/pattern142.txt")
                ),

                (
                    "pattern143",
                    Path::new("./presets/patterns/pattern143.txt")
                ),

                (
                    "pattern144",
                    Path::new("./presets/patterns/pattern144.txt")
                ),

                (
                    "pattern145",
                    Path::new("./presets/patterns/pattern145.txt")
                ),

                (
                    "pattern146",
                    Path::new("./presets/patterns/pattern146.txt")
                ),

                (
                    "pattern147",
                    Path::new("./presets/patterns/pattern147.txt")
                ),

                (
                    "pattern148",
                    Path::new("./presets/patterns/pattern148.txt")
                ),

                (
                    "pattern149",
                    Path::new("./presets/patterns/pattern149.txt")
                ),

                (
                    "pattern150",
                    Path::new("./presets/patterns/pattern150.txt")
                ),

                (
                    "pattern151",
                    Path::new("./presets/patterns/pattern151.txt")
                ),

                (
                    "pattern152",
                    Path::new("./presets/patterns/pattern152.txt")
                ),

                (
                    "pattern153",
                    Path::new("./presets/patterns/pattern153.txt")
                ),

                (
                    "pattern154",
                    Path::new("./presets/patterns/pattern154.txt")
                ),

                (
                    "pattern155",
                    Path::new("./presets/patterns/pattern155.txt")
                ),

                (
                    "pattern156",
                    Path::new("./presets/patterns/pattern156.txt")
                ),

                (
                    "pattern157",
                    Path::new("./presets/patterns/pattern157.txt")
                ),

                (
                    "pattern158",
                    Path::new("./presets/patterns/pattern158.txt")
                ),

                (
                    "pattern159",
                    Path::new("./presets/patterns/pattern159.txt")
                ),

                (
                    "pattern160",
                    Path::new("./presets/patterns/pattern160.txt")
                ),

                (
                    "pattern161",
                    Path::new("./presets/patterns/pattern161.txt")
                ),

                (
                    "pattern162",
                    Path::new("./presets/patterns/pattern162.txt")
                ),

                (
                    "pattern163",
                    Path::new("./presets/patterns/pattern163.txt")
                ),

                (
                    "pattern164",
                    Path::new("./presets/patterns/pattern164.txt")
                ),

                (
                    "pattern165",
                    Path::new("./presets/patterns/pattern165.txt")
                ),

                (
                    "pattern166",
                    Path::new("./presets/patterns/pattern166.txt")
                ),

                (
                    "pattern167",
                    Path::new("./presets/patterns/pattern167.txt")
                ),

                (
                    "pattern168",
                    Path::new("./presets/patterns/pattern168.txt")
                ),

                (
                    "pattern169",
                    Path::new("./presets/patterns/pattern169.txt")
                ),

                (
                    "pattern170",
                    Path::new("./presets/patterns/pattern170.txt")
                ),

                (
                    "pattern171",
                    Path::new("./presets/patterns/pattern171.txt")
                ),

                (
                    "pattern172",
                    Path::new("./presets/patterns/pattern172.txt")
                ),

                (
                    "pattern173",
                    Path::new("./presets/patterns/pattern173.txt")
                ),

                (
                    "pattern174",
                    Path::new("./presets/patterns/pattern174.txt")
                ),

                (
                    "pattern175",
                    Path::new("./presets/patterns/pattern175.txt")
                ),

                (
                    "pattern176",
                    Path::new("./presets/patterns/pattern176.txt")
                ),

                (
                    "pattern177",
                    Path::new("./presets/patterns/pattern177.txt")
                ),

                (
                    "pattern178",
                    Path::new("./presets/patterns/pattern178.txt")
                ),

                (
                    "pattern179",
                    Path::new("./presets/patterns/pattern179.txt")
                ),

                (
                    "pattern180",
                    Path::new("./presets/patterns/pattern180.txt")
                ),

                (
                    "pattern181",
                    Path::new("./presets/patterns/pattern181.txt")
                ),

                (
                    "pattern182",
                    Path::new("./presets/patterns/pattern182.txt")
                ),

                (
                    "pattern183",
                    Path::new("./presets/patterns/pattern183.txt")
                ),

                (
                    "pattern184",
                    Path::new("./presets/patterns/pattern184.txt")
                ),

                (
                    "pattern185",
                    Path::new("./presets/patterns/pattern185.txt")
                ),

                (
                    "pattern186",
                    Path::new("./presets/patterns/pattern186.txt")
                ),

                (
                    "pattern187",
                    Path::new("./presets/patterns/pattern187.txt")
                ),

                (
                    "pattern188",
                    Path::new("./presets/patterns/pattern188.txt")
                ),

                (
                    "pattern189",
                    Path::new("./presets/patterns/pattern189.txt")
                ),

                (
                    "pattern190",
                    Path::new("./presets/patterns/pattern190.txt")
                ),

                (
                    "pattern191",
                    Path::new("./presets/patterns/pattern191.txt")
                ),

                (
                    "pattern192",
                    Path::new("./presets/patterns/pattern192.txt")
                ),

                (
                    "pattern193",
                    Path::new("./presets/patterns/pattern193.txt")
                ),

                (
                    "pattern194",
                    Path::new("./presets/patterns/pattern194.txt")
                ),

                (
                    "pattern195",
                    Path::new("./presets/patterns/pattern195.txt")
                ),

                (
                    "pattern196",
                    Path::new("./presets/patterns/pattern196.txt")
                ),

                (
                    "pattern197",
                    Path::new("./presets/patterns/pattern197.txt")
                ),

                (
                    "pattern198",
                    Path::new("./presets/patterns/pattern198.txt")
                ),

                (
                    "pattern199",
                    Path::new("./presets/patterns/pattern199.txt")
                ),

                (
                    "pattern200",
                    Path::new("./presets/patterns/pattern200.txt")
                ),

                (
                    "pattern201",
                    Path::new("./presets/patterns/pattern201.txt")
                ),

                (
                    "pattern202",
                    Path::new("./presets/patterns/pattern202.txt")
                ),

                (
                    "pattern203",
                    Path::new("./presets/patterns/pattern203.txt")
                ),

                (
                    "pattern204",
                    Path::new("./presets/patterns/pattern204.txt")
                ),

                (
                    "pattern205",
                    Path::new("./presets/patterns/pattern205.txt")
                ),

                (
                    "pattern206",
                    Path::new("./presets/patterns/pattern206.txt")
                ),

                (
                    "pattern207",
                    Path::new("./presets/patterns/pattern207.txt")
                ),

                (
                    "pattern208",
                    Path::new("./presets/patterns/pattern208.txt")
                ),

                (
                    "pattern209",
                    Path::new("./presets/patterns/pattern209.txt")
                ),

                (
                    "pattern210",
                    Path::new("./presets/patterns/pattern210.txt")
                ),

                (
                    "pattern211",
                    Path::new("./presets/patterns/pattern211.txt")
                ),

                (
                    "pattern212",
                    Path::new("./presets/patterns/pattern212.txt")
                ),

                (
                    "pattern213",
                    Path::new("./presets/patterns/pattern213.txt")
                ),

                (
                    "pattern214",
                    Path::new("./presets/patterns/pattern214.txt")
                ),

                (
                    "pattern215",
                    Path::new("./presets/patterns/pattern215.txt")
                ),

                (
                    "pattern216",
                    Path::new("./presets/patterns/pattern216.txt")
                ),

                (
                    "pattern217",
                    Path::new("./presets/patterns/pattern217.txt")
                ),

                (
                    "pattern218",
                    Path::new("./presets/patterns/pattern218.txt")
                ),

                (
                    "pattern219",
                    Path::new("./presets/patterns/pattern219.txt")
                ),

                (
                    "pattern220",
                    Path::new("./presets/patterns/pattern220.txt")
                ),

                (
                    "pattern221",
                    Path::new("./presets/patterns/pattern221.txt")
                ),

                (
                    "pattern222",
                    Path::new("./presets/patterns/pattern222.txt")
                ),

                (
                    "pattern223",
                    Path::new("./presets/patterns/pattern223.txt")
                ),

                (
                    "pattern224",
                    Path::new("./presets/patterns/pattern224.txt")
                ),

                (
                    "pattern225",
                    Path::new("./presets/patterns/pattern225.txt")
                ),

                (
                    "pattern226",
                    Path::new("./presets/patterns/pattern226.txt")
                ),

                (
                    "pattern227",
                    Path::new("./presets/patterns/pattern227.txt")
                ),

                (
                    "pattern228",
                    Path::new("./presets/patterns/pattern228.txt")
                ),

                (
                    "pattern229",
                    Path::new("./presets/patterns/pattern229.txt")
                ),

                (
                    "pattern230",
                    Path::new("./presets/patterns/pattern230.txt")
                ),

                (
                    "pattern231",
                    Path::new("./presets/patterns/pattern231.txt")
                ),

                (
                    "pattern232",
                    Path::new("./presets/patterns/pattern232.txt")
                ),

                (
                    "pattern233",
                    Path::new("./presets/patterns/pattern233.txt")
                ),

                (
                    "pattern234",
                    Path::new("./presets/patterns/pattern234.txt")
                ),

                (
                    "pattern235",
                    Path::new("./presets/patterns/pattern235.txt")
                ),

                (
                    "pattern236",
                    Path::new("./presets/patterns/pattern236.txt")
                ),

                (
                    "pattern237",
                    Path::new("./presets/patterns/pattern237.txt")
                ),

                (
                    "pattern238",
                    Path::new("./presets/patterns/pattern238.txt")
                ),

                (
                    "pattern239",
                    Path::new("./presets/patterns/pattern239.txt")
                ),

                (
                    "pattern240",
                    Path::new("./presets/patterns/pattern240.txt")
                ),

                (
                    "pattern241",
                    Path::new("./presets/patterns/pattern241.txt")
                ),

                (
                    "pattern242",
                    Path::new("./presets/patterns/pattern242.txt")
                ),

                (
                    "pattern243",
                    Path::new("./presets/patterns/pattern243.txt")
                ),

                (
                    "pattern244",
                    Path::new("./presets/patterns/pattern244.txt")
                ),

                (
                    "pattern245",
                    Path::new("./presets/patterns/pattern245.txt")
                ),

                (
                    "pattern246",
                    Path::new("./presets/patterns/pattern246.txt")
                ),

                (
                    "pattern247",
                    Path::new("./presets/patterns/pattern247.txt")
                ),

                (
                    "pattern248",
                    Path::new("./presets/patterns/pattern248.txt")
                ),

                (
                    "pattern249",
                    Path::new("./presets/patterns/pattern249.txt")
                ),

                (
                    "pattern250",
                    Path::new("./presets/patterns/pattern250.txt")
                ),

                (
                    "pattern251",
                    Path::new("./presets/patterns/pattern251.txt")
                ),

                (
                    "pattern252",
                    Path::new("./presets/patterns/pattern252.txt")
                ),

                (
                    "pattern253",
                    Path::new("./presets/patterns/pattern253.txt")
                ),

                (
                    "pattern254",
                    Path::new("./presets/patterns/pattern254.txt")
                ),

                (
                    "pattern255",
                    Path::new("./presets/patterns/pattern255.txt")
                ),

                (
                    "pattern256",
                    Path::new("./presets/patterns/pattern256.txt")
                ),

                (
                    "pattern257",
                    Path::new("./presets/patterns/pattern257.txt")
                ),

                (
                    "pattern258",
                    Path::new("./presets/patterns/pattern258.txt")
                ),

                (
                    "pattern259",
                    Path::new("./presets/patterns/pattern259.txt")
                ),

                (
                    "pattern260",
                    Path::new("./presets/patterns/pattern260.txt")
                ),

                (
                    "pattern261",
                    Path::new("./presets/patterns/pattern261.txt")
                ),

                (
                    "pattern262",
                    Path::new("./presets/patterns/pattern262.txt")
                ),

                (
                    "pattern263",
                    Path::new("./presets/patterns/pattern263.txt")
                ),

                (
                    "pattern264",
                    Path::new("./presets/patterns/pattern264.txt")
                ),

                (
                    "pattern265",
                    Path::new("./presets/patterns/pattern265.txt")
                ),

                (
                    "pattern266",
                    Path::new("./presets/patterns/pattern266.txt")
                ),

                (
                    "pattern267",
                    Path::new("./presets/patterns/pattern267.txt")
                ),

                (
                    "pattern268",
                    Path::new("./presets/patterns/pattern268.txt")
                ),

                (
                    "pattern269",
                    Path::new("./presets/patterns/pattern269.txt")
                ),

                (
                    "pattern270",
                    Path::new("./presets/patterns/pattern270.txt")
                ),

                (
                    "pattern271",
                    Path::new("./presets/patterns/pattern271.txt")
                ),

                (
                    "pattern272",
                    Path::new("./presets/patterns/pattern272.txt")
                ),

                (
                    "pattern273",
                    Path::new("./presets/patterns/pattern273.txt")
                ),

                (
                    "pattern274",
                    Path::new("./presets/patterns/pattern274.txt")
                ),

                (
                    "pattern275",
                    Path::new("./presets/patterns/pattern275.txt")
                ),

                (
                    "pattern276",
                    Path::new("./presets/patterns/pattern276.txt")
                ),

                (
                    "pattern277",
                    Path::new("./presets/patterns/pattern277.txt")
                ),

                (
                    "pattern278",
                    Path::new("./presets/patterns/pattern278.txt")
                ),

                (
                    "pattern279",
                    Path::new("./presets/patterns/pattern279.txt")
                ),

                (
                    "pattern280",
                    Path::new("./presets/patterns/pattern280.txt")
                ),

                (
                    "pattern281",
                    Path::new("./presets/patterns/pattern281.txt")
                ),

                (
                    "pattern282",
                    Path::new("./presets/patterns/pattern282.txt")
                ),

                (
                    "pattern283",
                    Path::new("./presets/patterns/pattern283.txt")
                ),

                (
                    "pattern284",
                    Path::new("./presets/patterns/pattern284.txt")
                ),

                (
                    "pattern285",
                    Path::new("./presets/patterns/pattern285.txt")
                ),

                (
                    "pattern286",
                    Path::new("./presets/patterns/pattern286.txt")
                ),

                (
                    "pattern287",
                    Path::new("./presets/patterns/pattern287.txt")
                ),

                (
                    "pattern288",
                    Path::new("./presets/patterns/pattern288.txt")
                ),

                (
                    "pattern289",
                    Path::new("./presets/patterns/pattern289.txt")
                ),

                (
                    "pattern290",
                    Path::new("./presets/patterns/pattern290.txt")
                ),

                (
                    "pattern291",
                    Path::new("./presets/patterns/pattern291.txt")
                ),

                (
                    "pattern292",
                    Path::new("./presets/patterns/pattern292.txt")
                ),

                (
                    "pattern293",
                    Path::new("./presets/patterns/pattern293.txt")
                ),

                (
                    "pattern294",
                    Path::new("./presets/patterns/pattern294.txt")
                ),

                (
                    "pattern295",
                    Path::new("./presets/patterns/pattern295.txt")
                ),

                (
                    "pattern296",
                    Path::new("./presets/patterns/pattern296.txt")
                ),

                (
                    "pattern297",
                    Path::new("./presets/patterns/pattern297.txt")
                ),

                (
                    "pattern298",
                    Path::new("./presets/patterns/pattern298.txt")
                ),

                (
                    "pattern299",
                    Path::new("./presets/patterns/pattern299.txt")
                ),

                (
                    "pattern300",
                    Path::new("./presets/patterns/pattern300.txt")
                ),

                (
                    "pattern301",
                    Path::new("./presets/patterns/pattern301.txt")
                ),

                (
                    "pattern302",
                    Path::new("./presets/patterns/pattern302.txt")
                ),

                (
                    "pattern303",
                    Path::new("./presets/patterns/pattern303.txt")
                ),

                (
                    "pattern304",
                    Path::new("./presets/patterns/pattern304.txt")
                ),

                (
                    "pattern305",
                    Path::new("./presets/patterns/pattern305.txt")
                ),

                (
                    "pattern306",
                    Path::new("./presets/patterns/pattern306.txt")
                ),

                (
                    "pattern307",
                    Path::new("./presets/patterns/pattern307.txt")
                ),

                (
                    "pattern308",
                    Path::new("./presets/patterns/pattern308.txt")
                ),

                (
                    "pattern309",
                    Path::new("./presets/patterns/pattern309.txt")
                ),

                (
                    "pattern310",
                    Path::new("./presets/patterns/pattern310.txt")
                ),

                (
                    "pattern311",
                    Path::new("./presets/patterns/pattern311.txt")
                ),

                (
                    "pattern312",
                    Path::new("./presets/patterns/pattern312.txt")
                ),

                (
                    "pattern313",
                    Path::new("./presets/patterns/pattern313.txt")
                ),

                (
                    "pattern314",
                    Path::new("./presets/patterns/pattern314.txt")
                ),

                (
                    "pattern315",
                    Path::new("./presets/patterns/pattern315.txt")
                ),

                (
                    "pattern316",
                    Path::new("./presets/patterns/pattern316.txt")
                ),

                (
                    "pattern317",
                    Path::new("./presets/patterns/pattern317.txt")
                ),

                (
                    "pattern318",
                    Path::new("./presets/patterns/pattern318.txt")
                ),

                (
                    "pattern319",
                    Path::new("./presets/patterns/pattern319.txt")
                ),

                (
                    "pattern320",
                    Path::new("./presets/patterns/pattern320.txt")
                ),

                (
                    "pattern321",
                    Path::new("./presets/patterns/pattern321.txt")
                ),

                (
                    "pattern322",
                    Path::new("./presets/patterns/pattern322.txt")
                ),

                (
                    "pattern323",
                    Path::new("./presets/patterns/pattern323.txt")
                ),

                (
                    "pattern324",
                    Path::new("./presets/patterns/pattern324.txt")
                ),

                (
                    "pattern325",
                    Path::new("./presets/patterns/pattern325.txt")
                ),

                (
                    "pattern326",
                    Path::new("./presets/patterns/pattern326.txt")
                ),

                (
                    "pattern327",
                    Path::new("./presets/patterns/pattern327.txt")
                ),

                (
                    "pattern328",
                    Path::new("./presets/patterns/pattern328.txt")
                ),

                (
                    "pattern329",
                    Path::new("./presets/patterns/pattern329.txt")
                ),

                (
                    "pattern330",
                    Path::new("./presets/patterns/pattern330.txt")
                ),

                (
                    "pattern331",
                    Path::new("./presets/patterns/pattern331.txt")
                ),

                (
                    "pattern332",
                    Path::new("./presets/patterns/pattern332.txt")
                ),

                (
                    "pattern333",
                    Path::new("./presets/patterns/pattern333.txt")
                ),

                (
                    "pattern334",
                    Path::new("./presets/patterns/pattern334.txt")
                ),

                (
                    "pattern335",
                    Path::new("./presets/patterns/pattern335.txt")
                ),

                (
                    "pattern336",
                    Path::new("./presets/patterns/pattern336.txt")
                ),

                (
                    "pattern337",
                    Path::new("./presets/patterns/pattern337.txt")
                ),

                (
                    "pattern338",
                    Path::new("./presets/patterns/pattern338.txt")
                ),

                (
                    "pattern339",
                    Path::new("./presets/patterns/pattern339.txt")
                ),

                (
                    "pattern340",
                    Path::new("./presets/patterns/pattern340.txt")
                ),

                (
                    "pattern341",
                    Path::new("./presets/patterns/pattern341.txt")
                ),

                (
                    "pattern342",
                    Path::new("./presets/patterns/pattern342.txt")
                ),

                (
                    "pattern343",
                    Path::new("./presets/patterns/pattern343.txt")
                ),

                (
                    "pattern344",
                    Path::new("./presets/patterns/pattern344.txt")
                ),

                (
                    "pattern345",
                    Path::new("./presets/patterns/pattern345.txt")
                ),

                (
                    "pattern346",
                    Path::new("./presets/patterns/pattern346.txt")
                ),

                (
                    "pattern347",
                    Path::new("./presets/patterns/pattern347.txt")
                ),

                (
                    "pattern348",
                    Path::new("./presets/patterns/pattern348.txt")
                ),

                (
                    "pattern349",
                    Path::new("./presets/patterns/pattern349.txt")
                ),

                (
                    "pattern350",
                    Path::new("./presets/patterns/pattern350.txt")
                ),

                (
                    "pattern351",
                    Path::new("./presets/patterns/pattern351.txt")
                ),

                (
                    "pattern352",
                    Path::new("./presets/patterns/pattern352.txt")
                ),

                (
                    "pattern353",
                    Path::new("./presets/patterns/pattern353.txt")
                ),

                (
                    "pattern354",
                    Path::new("./presets/patterns/pattern354.txt")
                ),

                (
                    "pattern355",
                    Path::new("./presets/patterns/pattern355.txt")
                ),

                (
                    "pattern356",
                    Path::new("./presets/patterns/pattern356.txt")
                ),

                (
                    "pattern357",
                    Path::new("./presets/patterns/pattern357.txt")
                ),

                (
                    "pattern358",
                    Path::new("./presets/patterns/pattern358.txt")
                ),

                (
                    "pattern359",
                    Path::new("./presets/patterns/pattern359.txt")
                ),

                (
                    "pattern360",
                    Path::new("./presets/patterns/pattern360.txt")
                ),

                (
                    "pattern361",
                    Path::new("./presets/patterns/pattern361.txt")
                ),

                (
                    "pattern362",
                    Path::new("./presets/patterns/pattern362.txt")
                ),

                (
                    "pattern363",
                    Path::new("./presets/patterns/pattern363.txt")
                ),

                (
                    "pattern364",
                    Path::new("./presets/patterns/pattern364.txt")
                ),

                (
                    "pattern365",
                    Path::new("./presets/patterns/pattern365.txt")
                ),

                (
                    "pattern366",
                    Path::new("./presets/patterns/pattern366.txt")
                ),

                (
                    "pattern367",
                    Path::new("./presets/patterns/pattern367.txt")
                ),

                (
                    "pattern368",
                    Path::new("./presets/patterns/pattern368.txt")
                ),

                (
                    "pattern369",
                    Path::new("./presets/patterns/pattern369.txt")
                ),

                (
                    "pattern370",
                    Path::new("./presets/patterns/pattern370.txt")
                ),

                (
                    "pattern371",
                    Path::new("./presets/patterns/pattern371.txt")
                ),

                (
                    "pattern372",
                    Path::new("./presets/patterns/pattern372.txt")
                ),

                (
                    "pattern373",
                    Path::new("./presets/patterns/pattern373.txt")
                ),

                (
                    "pattern374",
                    Path::new("./presets/patterns/pattern374.txt")
                ),

                (
                    "pattern375",
                    Path::new("./presets/patterns/pattern375.txt")
                ),

                (
                    "pattern376",
                    Path::new("./presets/patterns/pattern376.txt")
                ),

                (
                    "pattern377",
                    Path::new("./presets/patterns/pattern377.txt")
                ),

                (
                    "pattern378",
                    Path::new("./presets/patterns/pattern378.txt")
                ),

                (
                    "pattern379",
                    Path::new("./presets/patterns/pattern379.txt")
                ),

                (
                    "pattern380",
                    Path::new("./presets/patterns/pattern380.txt")
                ),

                (
                    "pattern381",
                    Path::new("./presets/patterns/pattern381.txt")
                ),

                (
                    "pattern382",
                    Path::new("./presets/patterns/pattern382.txt")
                ),

                (
                    "pattern383",
                    Path::new("./presets/patterns/pattern383.txt")
                ),

                (
                    "pattern384",
                    Path::new("./presets/patterns/pattern384.txt")
                ),

                (
                    "pattern385",
                    Path::new("./presets/patterns/pattern385.txt")
                ),

                (
                    "pattern386",
                    Path::new("./presets/patterns/pattern386.txt")
                ),

                (
                    "pattern387",
                    Path::new("./presets/patterns/pattern387.txt")
                ),

                (
                    "pattern388",
                    Path::new("./presets/patterns/pattern388.txt")
                ),

                (
                    "pattern389",
                    Path::new("./presets/patterns/pattern389.txt")
                ),

                (
                    "pattern390",
                    Path::new("./presets/patterns/pattern390.txt")
                ),

                (
                    "pattern391",
                    Path::new("./presets/patterns/pattern391.txt")
                ),

                (
                    "pattern392",
                    Path::new("./presets/patterns/pattern392.txt")
                ),

                (
                    "pattern393",
                    Path::new("./presets/patterns/pattern393.txt")
                ),

                (
                    "pattern394",
                    Path::new("./presets/patterns/pattern394.txt")
                ),

                (
                    "pattern395",
                    Path::new("./presets/patterns/pattern395.txt")
                ),

                (
                    "pattern396",
                    Path::new("./presets/patterns/pattern396.txt")
                ),

                (
                    "pattern397",
                    Path::new("./presets/patterns/pattern397.txt")
                ),

                (
                    "pattern398",
                    Path::new("./presets/patterns/pattern398.txt")
                ),

                (
                    "pattern399",
                    Path::new("./presets/patterns/pattern399.txt")
                ),

                (
                    "pattern400",
                    Path::new("./presets/patterns/pattern400.txt")
                ),

                (
                    "pattern401",
                    Path::new("./presets/patterns/pattern401.txt")
                ),

                (
                    "pattern402",
                    Path::new("./presets/patterns/pattern402.txt")
                ),

                (
                    "pattern403",
                    Path::new("./presets/patterns/pattern403.txt")
                ),

                (
                    "pattern404",
                    Path::new("./presets/patterns/pattern404.txt")
                ),

                (
                    "pattern405",
                    Path::new("./presets/patterns/pattern405.txt")
                ),

                (
                    "pattern406",
                    Path::new("./presets/patterns/pattern406.txt")
                ),

                (
                    "pattern407",
                    Path::new("./presets/patterns/pattern407.txt")
                ),

                (
                    "pattern408",
                    Path::new("./presets/patterns/pattern408.txt")
                ),

                (
                    "pattern409",
                    Path::new("./presets/patterns/pattern409.txt")
                ),

                (
                    "pattern410",
                    Path::new("./presets/patterns/pattern410.txt")
                ),

                (
                    "pattern411",
                    Path::new("./presets/patterns/pattern411.txt")
                ),

                (
                    "pattern412",
                    Path::new("./presets/patterns/pattern412.txt")
                ),

                (
                    "pattern413",
                    Path::new("./presets/patterns/pattern413.txt")
                ),

                (
                    "pattern414",
                    Path::new("./presets/patterns/pattern414.txt")
                ),

                (
                    "pattern415",
                    Path::new("./presets/patterns/pattern415.txt")
                ),

                (
                    "pattern416",
                    Path::new("./presets/patterns/pattern416.txt")
                ),

                (
                    "pattern417",
                    Path::new("./presets/patterns/pattern417.txt")
                ),

                (
                    "pattern418",
                    Path::new("./presets/patterns/pattern418.txt")
                ),

                (
                    "pattern419",
                    Path::new("./presets/patterns/pattern419.txt")
                ),

                (
                    "pattern420",
                    Path::new("./presets/patterns/pattern420.txt")
                ),

                (
                    "pattern421",
                    Path::new("./presets/patterns/pattern421.txt")
                ),

                (
                    "pattern422",
                    Path::new("./presets/patterns/pattern422.txt")
                ),

                (
                    "pattern423",
                    Path::new("./presets/patterns/pattern423.txt")
                ),

                (
                    "pattern424",
                    Path::new("./presets/patterns/pattern424.txt")
                ),

                (
                    "pattern425",
                    Path::new("./presets/patterns/pattern425.txt")
                ),

                (
                    "pattern426",
                    Path::new("./presets/patterns/pattern426.txt")
                ),

                (
                    "pattern427",
                    Path::new("./presets/patterns/pattern427.txt")
                ),

                (
                    "pattern428",
                    Path::new("./presets/patterns/pattern428.txt")
                ),

                (
                    "pattern429",
                    Path::new("./presets/patterns/pattern429.txt")
                ),

                (
                    "pattern430",
                    Path::new("./presets/patterns/pattern430.txt")
                ),

                (
                    "pattern431",
                    Path::new("./presets/patterns/pattern431.txt")
                ),

                (
                    "pattern432",
                    Path::new("./presets/patterns/pattern432.txt")
                ),

                (
                    "pattern433",
                    Path::new("./presets/patterns/pattern433.txt")
                ),

                (
                    "pattern434",
                    Path::new("./presets/patterns/pattern434.txt")
                ),

                (
                    "pattern435",
                    Path::new("./presets/patterns/pattern435.txt")
                ),

                (
                    "pattern436",
                    Path::new("./presets/patterns/pattern436.txt")
                ),

                (
                    "pattern437",
                    Path::new("./presets/patterns/pattern437.txt")
                ),

                (
                    "pattern438",
                    Path::new("./presets/patterns/pattern438.txt")
                ),

                (
                    "pattern439",
                    Path::new("./presets/patterns/pattern439.txt")
                ),

                (
                    "pattern440",
                    Path::new("./presets/patterns/pattern440.txt")
                ),

                (
                    "pattern441",
                    Path::new("./presets/patterns/pattern441.txt")
                ),

                (
                    "pattern442",
                    Path::new("./presets/patterns/pattern442.txt")
                ),

                (
                    "pattern443",
                    Path::new("./presets/patterns/pattern443.txt")
                ),

                (
                    "pattern444",
                    Path::new("./presets/patterns/pattern444.txt")
                ),

                (
                    "pattern445",
                    Path::new("./presets/patterns/pattern445.txt")
                ),

                (
                    "pattern446",
                    Path::new("./presets/patterns/pattern446.txt")
                ),

                (
                    "pattern447",
                    Path::new("./presets/patterns/pattern447.txt")
                ),

                (
                    "pattern448",
                    Path::new("./presets/patterns/pattern448.txt")
                ),

                (
                    "pattern449",
                    Path::new("./presets/patterns/pattern449.txt")
                ),

                (
                    "pattern450",
                    Path::new("./presets/patterns/pattern450.txt")
                ),

                (
                    "pattern451",
                    Path::new("./presets/patterns/pattern451.txt")
                ),

                (
                    "pattern452",
                    Path::new("./presets/patterns/pattern452.txt")
                ),

                (
                    "pattern453",
                    Path::new("./presets/patterns/pattern453.txt")
                ),

                (
                    "pattern454",
                    Path::new("./presets/patterns/pattern454.txt")
                ),

                (
                    "pattern455",
                    Path::new("./presets/patterns/pattern455.txt")
                ),

                (
                    "pattern456",
                    Path::new("./presets/patterns/pattern456.txt")
                ),

                (
                    "pattern457",
                    Path::new("./presets/patterns/pattern457.txt")
                ),

                (
                    "pattern458",
                    Path::new("./presets/patterns/pattern458.txt")
                ),

                (
                    "pattern459",
                    Path::new("./presets/patterns/pattern459.txt")
                ),

                (
                    "pattern460",
                    Path::new("./presets/patterns/pattern460.txt")
                ),

                (
                    "pattern461",
                    Path::new("./presets/patterns/pattern461.txt")
                ),

                (
                    "pattern462",
                    Path::new("./presets/patterns/pattern462.txt")
                ),

                (
                    "pattern463",
                    Path::new("./presets/patterns/pattern463.txt")
                ),

                (
                    "pattern464",
                    Path::new("./presets/patterns/pattern464.txt")
                ),

                (
                    "pattern465",
                    Path::new("./presets/patterns/pattern465.txt")
                ),

                (
                    "pattern466",
                    Path::new("./presets/patterns/pattern466.txt")
                ),

                (
                    "pattern467",
                    Path::new("./presets/patterns/pattern467.txt")
                ),

                (
                    "pattern468",
                    Path::new("./presets/patterns/pattern468.txt")
                ),

                (
                    "pattern469",
                    Path::new("./presets/patterns/pattern469.txt")
                ),

                (
                    "pattern470",
                    Path::new("./presets/patterns/pattern470.txt")
                ),

                (
                    "pattern471",
                    Path::new("./presets/patterns/pattern471.txt")
                ),

                (
                    "pattern472",
                    Path::new("./presets/patterns/pattern472.txt")
                ),

                (
                    "pattern473",
                    Path::new("./presets/patterns/pattern473.txt")
                ),

                (
                    "pattern474",
                    Path::new("./presets/patterns/pattern474.txt")
                ),

                (
                    "pattern475",
                    Path::new("./presets/patterns/pattern475.txt")
                ),

                (
                    "pattern476",
                    Path::new("./presets/patterns/pattern476.txt")
                ),

                (
                    "pattern477",
                    Path::new("./presets/patterns/pattern477.txt")
                ),

                (
                    "pattern478",
                    Path::new("./presets/patterns/pattern478.txt")
                ),

                (
                    "pattern479",
                    Path::new("./presets/patterns/pattern479.txt")
                ),

                (
                    "pattern480",
                    Path::new("./presets/patterns/pattern480.txt")
                ),

                (
                    "pattern481",
                    Path::new("./presets/patterns/pattern481.txt")
                ),

                (
                    "pattern482",
                    Path::new("./presets/patterns/pattern482.txt")
                ),

                (
                    "pattern483",
                    Path::new("./presets/patterns/pattern483.txt")
                ),

                (
                    "pattern484",
                    Path::new("./presets/patterns/pattern484.txt")
                ),

                (
                    "pattern485",
                    Path::new("./presets/patterns/pattern485.txt")
                ),

                (
                    "pattern486",
                    Path::new("./presets/patterns/pattern486.txt")
                ),

                (
                    "pattern487",
                    Path::new("./presets/patterns/pattern487.txt")
                ),

                (
                    "pattern488",
                    Path::new("./presets/patterns/pattern488.txt")
                ),

                (
                    "pattern489",
                    Path::new("./presets/patterns/pattern489.txt")
                ),

                (
                    "pattern490",
                    Path::new("./presets/patterns/pattern490.txt")
                ),

                (
                    "pattern491",
                    Path::new("./presets/patterns/pattern491.txt")
                ),

                (
                    "pattern492",
                    Path::new("./presets/patterns/pattern492.txt")
                ),

                (
                    "pattern493",
                    Path::new("./presets/patterns/pattern493.txt")
                ),

                (
                    "pattern494",
                    Path::new("./presets/patterns/pattern494.txt")
                ),

                (
                    "pattern495",
                    Path::new("./presets/patterns/pattern495.txt")
                ),

                (
                    "pattern496",
                    Path::new("./presets/patterns/pattern496.txt")
                ),

                (
                    "pattern497",
                    Path::new("./presets/patterns/pattern497.txt")
                ),

                (
                    "pattern498",
                    Path::new("./presets/patterns/pattern498.txt")
                ),

                (
                    "pattern499",
                    Path::new("./presets/patterns/pattern499.txt")
                ),

                (
                    "pattern500",
                    Path::new("./presets/patterns/pattern500.txt")
                ),

                (
                    "pattern501",
                    Path::new("./presets/patterns/pattern501.txt")
                ),

                (
                    "pattern502",
                    Path::new("./presets/patterns/pattern502.txt")
                ),

                (
                    "pattern503",
                    Path::new("./presets/patterns/pattern503.txt")
                ),

                (
                    "pattern504",
                    Path::new("./presets/patterns/pattern504.txt")
                ),

                (
                    "pattern505",
                    Path::new("./presets/patterns/pattern505.txt")
                ),

                (
                    "pattern506",
                    Path::new("./presets/patterns/pattern506.txt")
                ),

                (
                    "pattern507",
                    Path::new("./presets/patterns/pattern507.txt")
                ),

                (
                    "pattern508",
                    Path::new("./presets/patterns/pattern508.txt")
                ),

                (
                    "pattern509",
                    Path::new("./presets/patterns/pattern509.txt")
                ),

                (
                    "pattern510",
                    Path::new("./presets/patterns/pattern510.txt")
                ),

                (
                    "pattern511",
                    Path::new("./presets/patterns/pattern511.txt")
                ),

                (
                    "pattern512",
                    Path::new("./presets/patterns/pattern512.txt")
                ),
            ]),
            flag_cur: false,
            layout: Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref()),
            cur_gen: Gen::new(),
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut item_cnt = 1;
    loop {
        terminal.draw(|f| ui_list(f, &mut app))?;
        let timeout = Duration::from_millis(32);
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left | KeyCode::Char('h') => app.items.unselect(),
                    KeyCode::Down | KeyCode::Char('j') => {
                        if item_cnt >= app.items.items.len() - 1 {
                            item_cnt = 0;
                        } else {
                            item_cnt += 1;
                        }
                        app.items.next();
                        app.cur_gen =
                            gen_from_file(app.items.items[item_cnt].1);
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if item_cnt == 0 {
                            item_cnt = app.items.items.len() - 1;
                        }
                        else {
                            item_cnt -= 1;
                        }
                        app.items.previous();
                        app.cur_gen =
                            gen_from_file(app.items.items[item_cnt].1);
                    }
                    KeyCode::Char('n') => {
                        terminal.draw(|f| ui_game(f, &mut app))?;
                    }
                    KeyCode::Char('a') => 'animate: loop {
                        terminal.draw(|f| ui_game(f, &mut app))?;
                        sleep(Duration::from_millis(32));
                        if (crossterm::event::poll(Duration::from_millis(1))).unwrap() {
                            if let Event::Key(k) = event::read().unwrap() {
                                match k.code {
                                    KeyCode::Char('s') => break 'animate,
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

fn ui_list<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = app.layout.split(f.size());

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.0)];
            ListItem::new(lines).style(Style::default().fg(Color::Red).bg(Color::Black))
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
        app.cur_gen = new_gen(&chunks[1], app);
    }
    let spans = gen_to_spans(&app.cur_gen);
    render_gen(f, chunks[1], &spans);
}

fn ui_game<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = app.layout.split(f.size());

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
        .block(Block::default().borders(Borders::ALL).title("Cool Patterns"))
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
