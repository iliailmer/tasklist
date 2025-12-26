use crate::manager::Mngr;
use crate::task::{Status, Task};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;

pub struct App {
    manager: Mngr,
    tasks: Vec<Task>,
    list_state: ListState,
    mode: AppMode,
    input: String,
    error_message: Option<String>,
}

#[derive(PartialEq)]
enum AppMode {
    Normal,
    AddingTask,
    ConfirmDelete,
}

impl App {
    pub fn new(manager: Mngr) -> io::Result<App> {
        let tasks = manager.get_tasks()?;
        let mut list_state = ListState::default();
        if !tasks.is_empty() {
            list_state.select(Some(0));
        }
        Ok(App {
            manager,
            tasks,
            list_state,
            mode: AppMode::Normal,
            input: String::new(),
            error_message: None,
        })
    }

    fn reload_tasks(&mut self) -> io::Result<()> {
        self.tasks = self.manager.get_tasks()?;
        if self.tasks.is_empty() {
            self.list_state.select(None);
        } else if let Some(selected) = self.list_state.selected() {
            if selected >= self.tasks.len() {
                self.list_state.select(Some(self.tasks.len() - 1));
            }
        } else {
            self.list_state.select(Some(0));
        }
        Ok(())
    }

    fn next(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.tasks.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn update_task_status(&mut self, status: Status) -> io::Result<()> {
        if let Some(selected) = self.list_state.selected() {
            if let Some(task) = self.tasks.get(selected) {
                self.manager
                    .update_task(task.id, status, None)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                self.reload_tasks()?;
            }
        }
        Ok(())
    }

    fn delete_current_task(&mut self) -> io::Result<()> {
        if let Some(selected) = self.list_state.selected() {
            if let Some(task) = self.tasks.get(selected) {
                self.manager
                    .delete_task(task.id)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                self.reload_tasks()?;
            }
        }
        Ok(())
    }

    fn add_task(&mut self) -> io::Result<()> {
        if !self.input.is_empty() {
            self.manager
                .add_task(self.input.clone())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            self.input.clear();
            self.mode = AppMode::Normal;
            self.reload_tasks()?;
            // Select the newly added task (last one)
            if !self.tasks.is_empty() {
                self.list_state.select(Some(self.tasks.len() - 1));
            }
        }
        Ok(())
    }
}

pub fn run(manager: Mngr) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let app = App::new(manager)?;
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                AppMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(())
                    }
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('n') => {
                        app.mode = AppMode::AddingTask;
                        app.input.clear();
                        app.error_message = None;
                    }
                    KeyCode::Char('d') => {
                        if app.list_state.selected().is_some() && !app.tasks.is_empty() {
                            app.mode = AppMode::ConfirmDelete;
                            app.error_message = None;
                        }
                    }
                    KeyCode::Char('1') => {
                        if let Err(e) = app.update_task_status(Status::NotStarted) {
                            app.error_message = Some(format!("Error: {}", e));
                        }
                    }
                    KeyCode::Char('2') => {
                        if let Err(e) = app.update_task_status(Status::InProgress) {
                            app.error_message = Some(format!("Error: {}", e));
                        }
                    }
                    KeyCode::Char('3') => {
                        if let Err(e) = app.update_task_status(Status::Done) {
                            app.error_message = Some(format!("Error: {}", e));
                        }
                    }
                    KeyCode::Char('r') => {
                        if let Err(e) = app.reload_tasks() {
                            app.error_message = Some(format!("Error reloading: {}", e));
                        } else {
                            app.error_message = None;
                        }
                    }
                    _ => {}
                },
                AppMode::AddingTask => match key.code {
                    KeyCode::Enter => {
                        if let Err(e) = app.add_task() {
                            app.error_message = Some(format!("Error: {}", e));
                            app.mode = AppMode::Normal;
                        }
                    }
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                        app.input.clear();
                        app.error_message = None;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
                AppMode::ConfirmDelete => match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        if let Err(e) = app.delete_current_task() {
                            app.error_message = Some(format!("Error: {}", e));
                        }
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                        app.error_message = None;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(5),    // Task list
            Constraint::Length(8), // Help
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            "TaskList - Interactive TUI",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            format!("Total tasks: {}", app.tasks.len()),
            Style::default().fg(Color::DarkGray),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Task list
    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| {
            let status_color = match task.status {
                Status::NotStarted => Color::Yellow,
                Status::InProgress => Color::Blue,
                Status::Done => Color::Green,
            };

            let content = Line::from(vec![
                Span::styled(
                    format!("{:3} ", task.id),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(format!("{} ", task.status), Style::default().fg(status_color)),
                Span::raw(&task.description),
                Span::styled(
                    format!(" ({})", task.date),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            ListItem::new(content)
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(match app.mode {
            AppMode::Normal => "Tasks (↑↓/jk: navigate, 1/2/3: status, n: new, d: delete, r: reload, q: quit)",
            AppMode::AddingTask => "Adding Task (Enter: save, Esc: cancel)",
            AppMode::ConfirmDelete => "Delete task? (y/n)",
        }))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks[1], &mut app.list_state);

    // Input or Help
    match app.mode {
        AppMode::AddingTask => {
            let input = Paragraph::new(app.input.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("New Task Description"),
                );
            f.render_widget(input, chunks[2]);
        }
        _ => {
            let help_text = vec![
                Line::from(vec![
                    Span::styled("Navigation: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("↑/k up, ↓/j down"),
                ]),
                Line::from(vec![
                    Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("1", Style::default().fg(Color::Yellow)),
                    Span::raw(" Not Started, "),
                    Span::styled("2", Style::default().fg(Color::Blue)),
                    Span::raw(" In Progress, "),
                    Span::styled("3", Style::default().fg(Color::Green)),
                    Span::raw(" Done"),
                ]),
                Line::from(vec![
                    Span::styled("Actions: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("n new task, d delete, r reload"),
                ]),
                Line::from(vec![
                    Span::styled("Exit: ", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("q or Ctrl+C"),
                ]),
            ];

            if let Some(error) = &app.error_message {
                let mut help_with_error = help_text;
                help_with_error.insert(
                    0,
                    Line::from(vec![Span::styled(
                        error,
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    )]),
                );
                let help = Paragraph::new(help_with_error)
                    .block(Block::default().borders(Borders::ALL).title("Help"))
                    .wrap(Wrap { trim: true });
                f.render_widget(help, chunks[2]);
            } else {
                let help = Paragraph::new(help_text)
                    .block(Block::default().borders(Borders::ALL).title("Help"))
                    .wrap(Wrap { trim: true });
                f.render_widget(help, chunks[2]);
            }
        }
    }
}
