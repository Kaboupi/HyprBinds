use crate::{Commands, constants};
use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

#[derive(Debug)]
enum InputMode {
    Normal,
    Editing,
}

pub fn show_commands(commands: Commands) {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut table_state = TableState::default();
    if !commands.commands.is_empty() {
        table_state.select(Some(0));
    }

    let mut input_mode = InputMode::Normal;
    let mut input = String::new();

    loop {
        let filtered_commands: Vec<_> = commands
            .commands
            .iter()
            .filter(|map| {
                if input.is_empty() {
                    return true;
                }
                let search_term = input.to_lowercase();
                map.values()
                    .any(|val| val.to_lowercase().contains(&search_term))
            })
            .collect();

        if let Some(selected) = table_state.selected() {
            if filtered_commands.is_empty() {
                table_state.select(None);
            } else if selected >= filtered_commands.len() {
                table_state.select(Some(filtered_commands.len() - 1));
            }
        } else if !filtered_commands.is_empty() {
            table_state.select(Some(0));
        }

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(0), Constraint::Length(3)])
                    .split(f.area());

                let headers_vec =
                    Vec::from([constants::K_KB.to_string(), constants::K_DESC.to_string()]);
                let headers_vec_alias =
                    Vec::from(["Keybind".to_string(), "Description".to_string()]);

                let header_row = Row::new(headers_vec_alias.iter().map(|h| {
                    Cell::from(h.as_str()).style(Style::default().add_modifier(Modifier::BOLD))
                }))
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1);

                let rows: Vec<Row> = filtered_commands
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
                    .block(Block::default().borders(Borders::ALL).title(
                        " Hyprland Binds ([q] to exit, [/] to search, ↑/↓ or j/k to scroll) ",
                    ))
                    .row_highlight_style(
                        Style::default()
                            .bg(Color::Indexed(237))
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                f.render_stateful_widget(table, chunks[0], &mut table_state);

                let search_title = match input_mode {
                    InputMode::Normal => " Search (Press '/' to type) ",
                    InputMode::Editing => " Search (Press 'Enter' or 'Esc' to finish) ",
                };

                let search_style = match input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => Style::default().fg(Color::Yellow),
                };

                let search_block = Paragraph::new(input.as_str())
                    .style(search_style)
                    .block(Block::default().borders(Borders::ALL).title(search_title));

                f.render_widget(search_block, chunks[1]);

                if let InputMode::Editing = input_mode {
                    // TODO: Replace with `set_cursor_position()`
                    #[allow(deprecated)]
                    f.set_cursor(
                        chunks[1].x + input.chars().count() as u16 + 1,
                        chunks[1].y + 1,
                    );
                }
            })
            .unwrap();

        if event::poll(std::time::Duration::from_millis(16)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('/') => {
                            input_mode = InputMode::Editing;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if let Some(selected) = table_state.selected() {
                                if selected < filtered_commands.len().saturating_sub(1) {
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
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter | KeyCode::Esc => {
                            input_mode = InputMode::Normal;
                            input.clear();
                        }
                        KeyCode::Char(c) => {
                            input.push(c);
                        }
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}
