//! Visual TUI dashboard extension
//!
//! Provides a terminal UI with service status, logs, and metrics

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use devkit_core::{AppContext, Extension, MenuItem};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

pub struct DashboardExtension;

impl Extension for DashboardExtension {
    fn name(&self) -> &str {
        "dashboard"
    }

    fn is_available(&self, _ctx: &AppContext) -> bool {
        true // Always available
    }

    fn menu_items(&self, _ctx: &AppContext) -> Vec<MenuItem> {
        vec![MenuItem {
            label: "📊 Open visual dashboard".to_string(),
                group: None,
            handler: Box::new(|ctx| run_dashboard(ctx).map_err(Into::into)),
        }]
    }
}

/// Run the TUI dashboard
pub fn run_dashboard(ctx: &AppContext) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the dashboard
    let res = run_app(&mut terminal, ctx);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    _ctx: &AppContext,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Min(10),   // Main content
                    Constraint::Length(3), // Footer
                ])
                .split(f.area());

            // Header
            let header = Paragraph::new("devkit Dashboard")
                .style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default().borders(Borders::ALL).title("Header"));
            f.render_widget(header, chunks[0]);

            // Main content - split into left and right
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            // Services panel (left)
            let services = vec![
                ListItem::new(Line::from(vec![
                    Span::styled("✓ ", Style::default().fg(Color::Green)),
                    Span::raw("Docker"),
                ])),
                ListItem::new(Line::from(vec![
                    Span::styled("✓ ", Style::default().fg(Color::Green)),
                    Span::raw("Postgres"),
                ])),
                ListItem::new(Line::from(vec![
                    Span::styled("✗ ", Style::default().fg(Color::Red)),
                    Span::raw("Redis"),
                ])),
            ];
            let services_list =
                List::new(services).block(Block::default().borders(Borders::ALL).title("Services"));
            f.render_widget(services_list, main_chunks[0]);

            // Logs panel (right)
            let logs = Paragraph::new("Logs would appear here...\nPress 'q' to quit")
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Logs"));
            f.render_widget(logs, main_chunks[1]);

            // Footer
            let footer = Paragraph::new("q: Quit | r: Refresh | c: Clear")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Help"));
            f.render_widget(footer, chunks[2]);
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => {
                        // Refresh logic
                    }
                    KeyCode::Char('c') => {
                        // Clear logs
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Dashboard state
pub struct DashboardState {
    pub services: Vec<ServiceStatus>,
    pub logs: Vec<String>,
    pub selected_panel: usize,
}

pub struct ServiceStatus {
    pub name: String,
    pub status: ServiceState,
    pub uptime: Option<u64>,
}

pub enum ServiceState {
    Running,
    Stopped,
    Error,
}

impl DashboardState {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            logs: Vec::new(),
            selected_panel: 0,
        }
    }

    pub fn refresh(&mut self) {
        // Refresh service status
        // Query Docker, databases, etc.
    }
}
