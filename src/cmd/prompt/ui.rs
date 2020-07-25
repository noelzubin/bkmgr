use crate::cmd::prompt::App;
use std::slice::Iter;
use std::iter::Map;
use std::fmt::Display;
use tui::{
  Frame,
  backend::TermionBackend,
  backend::Backend,
  layout::{Constraint, Direction, Layout, Rect, Alignment},
  style::{Color, Modifier, Style},
  text::{Span, Spans, Text},
  widgets::{Block, Borders, List, ListItem, Paragraph, Table, Row, Widget, Wrap, TableState },
  Terminal,
};

pub fn draw_input<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
  let input = Paragraph::new(app.input.as_ref())
    .style(Style::default().fg(Color::Yellow))
    .block(Block::default().borders(Borders::ALL).title("Input"));
  
  f.render_widget(input, chunk);
}

pub fn draw_list<B: Backend>(f: &mut Frame<B>, app: &mut App, chunk: Rect) {
  let bookmarks: Vec<Vec<String>> = app.bookmarks.iter().map(|bm|{
    vec![bm.title.clone(), bm.url.clone(), bm.tags.join(", ").clone()] 
  }).collect();


  let selected_style = Style::default()
  .fg(Color::Yellow)
  .add_modifier(Modifier::BOLD);
  let row_style = Style::default().fg(Color::White);

  let list = Table::new(
        ["Title", "Url", "Tags"].iter(),
        bookmarks.iter()
          .map(|bm| Row::StyledData(bm.iter(), row_style))
    )
    .block(Block::default().title("Table").borders(Borders::ALL))
    .header_style(Style::default().fg(Color::Yellow))
    .highlight_style(selected_style)
    .highlight_symbol("> ")
    .widths(&[Constraint::Percentage(35), Constraint::Percentage(40), Constraint::Percentage(25)]);
    // .style(Style::default().fg(Color::White))
    // .column_spacing(1);
  
  f.render_stateful_widget(list, chunk, &mut app.table_state);
}