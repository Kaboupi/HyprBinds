use crate::extract_bind;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[allow(unused_imports)]
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
#[allow(unused_imports)]
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
};

#[derive(Debug)]
pub struct Commands {
    pub commands: Vec<HashMap<&'static str, String>>,
}

impl Commands {
    pub fn new(items: Vec<HashMap<&'static str, String>>) -> Commands {
        Commands { commands: items }
    }

    pub fn from_reader<T: BufRead + Sized>(reader: T) -> Commands {
        let commands: Vec<HashMap<&'static str, String>> = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| extract_bind(&line))
            .collect();

        Commands::new(commands)
    }

    pub fn show(&self) {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut table_state = TableState::default();
        if !self.commands.is_empty() {
            table_state.select(Some(0));
        }

        loop {
            terminal
                .draw(|f| {
                    let rects = Layout::default()
                        .constraints([Constraint::Percentage(100)])
                        .split(f.area());

                    let headers_vec = if let Some(first) = self.commands.first() {
                        first.keys().map(|k| k.to_string()).collect::<Vec<_>>()
                    } else {
                        vec!["Key".to_string(), "Value".to_string()]
                    };

                    let header_row = Row::new(headers_vec.iter().map(|h| {
                        Cell::from(h.as_str()).style(Style::default().add_modifier(Modifier::BOLD))
                    }))
                    .style(Style::default().fg(Color::Yellow))
                    .bottom_margin(1);

                    let rows: Vec<Row> = self
                        .commands
                        .iter()
                        .map(|map| {
                            let cells = headers_vec.iter().map(|header| {
                                let val = map.get(header.as_str()).cloned().unwrap_or_default();
                                Cell::from(val)
                            });
                            Row::new(cells)
                        })
                        .collect();

                    let widths: Vec<Constraint> = headers_vec
                        .iter()
                        .map(|_| Constraint::Percentage(100 / headers_vec.len() as u16))
                        .collect();

                    let table = Table::new(rows, widths)
                        .header(header_row)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(" Hyprland Binds ([q] to exit, ↑/↓ or j/k to scroll) "),
                        )
                        .row_highlight_style(
                            Style::default()
                                .bg(Color::Indexed(237))
                                .add_modifier(Modifier::BOLD),
                        )
                        .highlight_symbol(">> ");

                    f.render_stateful_widget(table, rects[0], &mut table_state);
                })
                .unwrap();

            if event::poll(std::time::Duration::from_millis(16)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down | KeyCode::Char('j') => {
                            if let Some(selected) = table_state.selected() {
                                if selected < self.commands.len() - 1 {
                                    table_state.select(Some(selected + 1));
                                }
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if let Some(selected) = table_state.selected() {
                                if selected > 0 {
                                    table_state.select(Some(selected - 1));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        disable_raw_mode().unwrap();
        execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        terminal.show_cursor().unwrap();
    }
}
