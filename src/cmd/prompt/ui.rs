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
  widgets::{Block, Borders, List, ListItem, Paragraph, Table, Row, Widget, Wrap },
  Terminal,
};

pub fn draw_input<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
  let input = Paragraph::new(app.input.as_ref())
    .style(Style::default().fg(Color::Yellow))
    .block(Block::default().borders(Borders::ALL).title("Input"));
  
  f.render_widget(input, chunk);
}

pub fn draw_list<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {
  let bookmarks: Vec<Vec<String>> = app.bookmarks.iter().map(|bm|{
    vec![bm.title.clone(), bm.url.clone(), bm.tags.join(", ").clone()] 
  }).collect();

  let row_style = Style::default().fg(Color::White);
  let list = Table::new(
        ["Title", "Url", "Tags"].iter(),
        bookmarks.iter()
          .map(|bm| Row::StyledData(bm.iter(), row_style))
    )
    .block(Block::default().title("Table").borders(Borders::ALL))
    .header_style(Style::default().fg(Color::Yellow))
    .widths(&[Constraint::Percentage(35), Constraint::Percentage(40), Constraint::Percentage(25)])
    .style(Style::default().fg(Color::White))
    .column_spacing(1);
  
  f.render_widget(list, chunk);
}

pub fn draw_commandline<B: Backend>(f: &mut Frame<B>, app: &App, chunk: Rect) {

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    .split(chunk);

  let text = Spans::from(vec![
      Span::raw(":help for help"),
      Span::styled("   line",Style::default().add_modifier(Modifier::ITALIC)),
      Span::raw("."),
  ]);

  let line = Paragraph::new(text.clone())
    .style(Style::default().fg(Color::Black).bg(Color::Gray))
    .wrap(Wrap { trim: true });

  let line2 = Paragraph::new(text)
    .style(Style::default().fg(Color::Black).bg(Color::Gray))
    .alignment(Alignment::Right)
    .wrap(Wrap { trim: true });

  f.render_widget(line, chunks[0]);
  f.render_widget(line2, chunks[1]);
}