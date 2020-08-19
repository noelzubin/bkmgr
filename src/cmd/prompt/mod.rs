mod event;
mod ui;

use std::io;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use event::{Event, Events};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use crate::bookmark::Bookmark;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{TableState},
    Terminal,
};
use crate::cmd::search;
use crate::cmd::delete;

pub struct App {
  input: String,
  bookmarks: Vec<Bookmark>,
  event_publisher: mpsc::Sender<Event<Key>>,
  table_state: TableState,
}

impl App {
  fn new(event_publisher: mpsc::Sender<Event<Key>>)-> Self {
    Self {
      input: String::new(),
      bookmarks: Vec::new(),
      event_publisher,
      table_state: TableState::default(),
    }
  }

  fn search(&mut self) {
    let input = self.input.clone();
    let tx = self.event_publisher.clone();
    thread::spawn(move || {
      let bookmarks = search::search(input.clone().split_whitespace().map(String::from).collect());
      tx.send(Event::DB(event::DB::Bookmarks(bookmarks)))
    });
  }

  fn init(&mut self) {
    self.search();
  }
}

pub fn execute() -> Result<(), io::Error>{
  // Terminal initialization
  let stdout = io::stdout().into_raw_mode()?;
  let stdout = MouseTerminal::from(stdout);
  let stdout = AlternateScreen::from(stdout);
  let backend = TermionBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // events channel
  let (tx, rx) = mpsc::channel();

  // Setup event handlers
  let events = Events::new(tx.clone(), rx);

  // Create default app state
  let mut app = App::new(tx);
  app.init();
  let app = Arc::new(Mutex::new(app));

  loop {
    terminal.draw(|f| {
      let mut app = app.lock().unwrap();
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

        ui::draw_input(f, &app, chunks[0]);
        ui::draw_list(f, &mut app, chunks[1]);

        f.set_cursor(
          chunks[0].x + app.input.len() as u16 + 1,
          chunks[0].y + 1,
        );
    }).unwrap();

    // handle events
    {
      let mut app = app.lock().unwrap();

      match events.next().unwrap() {
        Event::Input(input) => {
          match input {
            Key::Up | Key::Ctrl('k') => {
              match app.table_state.selected() {
                None => {
                  if app.bookmarks.len() > 0 {
                    let new_ind = app.bookmarks.len() - 1;
                    app.table_state.select(Some(new_ind))
                  }
                },
                Some(i) => {
                  let new_ind = if i == 0 { app.bookmarks.len() -1 } else { i - 1 };
                  app.table_state.select(Some(new_ind))
                },
              }
            },
            Key::Down | Key::Ctrl('j') => {
              match app.table_state.selected() {
                None => {
                  if app.bookmarks.len() > 0 {
                    app.table_state.select(Some(0))
                  }
                },
                Some(i) => {
                  let new_ind = (i + 1) % app.bookmarks.len();
                  app.table_state.select(Some(new_ind));
                },
              }
            },
            Key::Char(c) => { 
              app.input.push(c);
              app.search();
            },
            Key::Backspace => {
              app.input.pop();
              app.search();
            },
            Key::Ctrl('o') => {
              if let Some(i) = app.table_state.selected() {
                let url = &app.bookmarks.get(i).unwrap().url;
                webbrowser::open(&url).unwrap();
              }
            },
            Key::Ctrl('d') => {
              if let Some(i) = app.table_state.selected() {
                let ids = vec![app.bookmarks.get(i).unwrap().id];
                delete::execute(ids);
                app.bookmarks.remove(i);
              }
            },
            Key::Ctrl('c') => { break },
            _ => {},
          }
        },
        Event::DB(event::DB::Bookmarks(bookmarks)) => {
          app.table_state.select(None);
          app.bookmarks = bookmarks;
        }
        _ => {}
      }
    }
  };

  Ok(())
}