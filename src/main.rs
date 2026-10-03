use std::error::Error;
use std::io::{self, Stdout};
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use mambo_tools::catalog::{PACKAGES, Package};
use mambo_tools::paths::install_root;
use mambo_tools::repo::{self, LocalState};
use mambo_ui::{Panel, Shell, Status, StatusLine, Theme, ratatui};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Cell, Clear, Paragraph, Row, Table, TableState, Wrap};
use ratatui::{Frame, Terminal};

type AppTerminal = Terminal<CrosstermBackend<Stdout>>;

fn main() {
    if let Err(error) = run() {
        eprintln!("mambo-tools: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let root = install_root().map_err(io::Error::other)?;
    enable_raw_mode()?;
    let _restore_terminal = RestoreTerminal;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;
    run_app(&mut terminal, App::new(root))?;
    terminal.show_cursor()?;
    Ok(())
}

struct RestoreTerminal;

impl Drop for RestoreTerminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

struct App {
    root: PathBuf,
    selected: usize,
    message: String,
    status: Status,
    confirming_delete: bool,
}

impl App {
    fn new(root: PathBuf) -> Self {
        Self {
            message: format!("Managing repositories in {}", root.display()),
            root,
            selected: 0,
            status: Status::Info,
            confirming_delete: false,
        }
    }

    fn package(&self) -> Package {
        PACKAGES[self.selected]
    }

    fn select_previous(&mut self) {
        self.selected = self.selected.checked_sub(1).unwrap_or(PACKAGES.len() - 1);
    }

    fn select_next(&mut self) {
        self.selected = (self.selected + 1) % PACKAGES.len();
    }
}

#[derive(Clone, Copy)]
enum Action {
    Install,
    Update,
    Delete,
}

fn run_app(terminal: &mut AppTerminal, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, &app))?;
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if !matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
            continue;
        }
        if key.kind == KeyEventKind::Repeat
            && !matches!(
                key.code,
                KeyCode::Up | KeyCode::Down | KeyCode::Char('j' | 'k')
            )
        {
            continue;
        }
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return Ok(());
        }

        if app.confirming_delete {
            match key.code {
                KeyCode::Char('y') => {
                    app.confirming_delete = false;
                    perform(terminal, &mut app, Action::Delete)?;
                }
                KeyCode::Char('n') | KeyCode::Esc => {
                    app.confirming_delete = false;
                    "Deletion cancelled".clone_into(&mut app.message);
                    app.status = Status::Info;
                }
                _ => {}
            }
            continue;
        }

        if should_quit(key) {
            return Ok(());
        }
        let action = match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.select_previous();
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.select_next();
                None
            }
            KeyCode::Char('i') | KeyCode::Enter => request(&mut app, Action::Install),
            KeyCode::Char('u') => request(&mut app, Action::Update),
            KeyCode::Char('d') => request(&mut app, Action::Delete),
            KeyCode::Char('r') => {
                "Local state refreshed".clone_into(&mut app.message);
                app.status = Status::Info;
                None
            }
            _ => None,
        };

        if let Some(action) = action {
            perform(terminal, &mut app, action)?;
        }
    }
}

fn should_quit(key: KeyEvent) -> bool {
    key.code == KeyCode::Char('q')
        || key.code == KeyCode::Esc
        || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
}

fn request(app: &mut App, action: Action) -> Option<Action> {
    let package = app.package();
    let state = repo::local_state(&app.root, package);
    match (action, state) {
        (Action::Install, LocalState::Missing) | (Action::Update, LocalState::Installed) => {
            Some(action)
        }
        (Action::Delete, LocalState::Installed) => {
            app.confirming_delete = true;
            app.status = Status::Warning;
            None
        }
        (Action::Install, LocalState::Installed) => {
            app.message = format!("{} is already installed; press u to update", package.name);
            app.status = Status::Info;
            None
        }
        (Action::Update | Action::Delete, LocalState::Missing) => {
            app.message = format!("{} is not installed", package.name);
            app.status = Status::Warning;
            None
        }
        (_, LocalState::Conflict) => {
            app.message = format!(
                "{} has a path conflict; move that path before continuing",
                package.name
            );
            app.status = Status::Error;
            None
        }
    }
}

fn pending_message(action: Action, package: Package) -> String {
    match action {
        Action::Install => format!("Installing {}…", package.name),
        Action::Update => format!("Updating {}…", package.name),
        Action::Delete => format!("Deleting {}…", package.name),
    }
}

fn perform(terminal: &mut AppTerminal, app: &mut App, action: Action) -> io::Result<()> {
    let package = app.package();
    app.message = pending_message(action, package);
    app.status = Status::Info;
    terminal.draw(|frame| draw(frame, app))?;

    let result = match action {
        Action::Install => repo::install(&app.root, package),
        Action::Update => repo::update(&app.root, package),
        Action::Delete => repo::delete(&app.root, package),
    };
    match result {
        Ok(message) => {
            app.message = message;
            app.status = Status::Success;
        }
        Err(error) => {
            app.message = error.to_string();
            app.status = Status::Error;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame<'_>, app: &App) {
    let theme = Theme::default();
    let content = Shell::new("MamboTools")
        .subtitle("Project Mambo repository manager")
        .footer("↑/↓ move  i install  u update  d delete  r refresh  q quit")
        .render(frame, &theme);

    let sections = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(3),
        Constraint::Length(1),
    ])
    .split(content);
    frame.render_widget(
        Paragraph::new(format!("Install root: {}", app.root.display())).style(theme.muted()),
        sections[0],
    );

    let repository_panel = Panel::new("Repositories").focused(true).block(&theme);
    let table_area = repository_panel.inner(sections[1]);
    frame.render_widget(repository_panel, sections[1]);

    let show_summary = table_area.width >= 70;
    let rows = PACKAGES.iter().copied().map(|package| {
        let state = repo::local_state(&app.root, package);
        let status = Cell::from(state.label()).style(state_style(state, &theme));
        let mut cells = vec![Cell::from(package.name), status];
        if show_summary {
            cells.push(Cell::from(package.summary));
        }
        Row::new(cells)
    });
    let widths = if show_summary {
        vec![
            Constraint::Length(18),
            Constraint::Length(15),
            Constraint::Min(20),
        ]
    } else {
        vec![Constraint::Percentage(55), Constraint::Percentage(45)]
    };
    let mut headings = vec![Cell::from("Repository"), Cell::from("State")];
    if show_summary {
        headings.push(Cell::from("Purpose"));
    }
    let table = Table::new(rows, widths)
        .header(
            Row::new(headings)
                .style(theme.base().add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .style(theme.base())
        .row_highlight_style(theme.selected())
        .highlight_symbol("› ");
    let mut table_state = TableState::default().with_selected(Some(app.selected));
    frame.render_stateful_widget(table, table_area, &mut table_state);

    frame.render_widget(
        StatusLine::new(app.status, app.message.as_str(), &theme),
        sections[2],
    );

    if app.confirming_delete {
        draw_delete_confirmation(frame, app.package(), &theme);
    }
}

fn state_style(state: LocalState, theme: &Theme) -> Style {
    match state {
        LocalState::Missing => theme.muted(),
        LocalState::Installed => theme.base().fg(theme.success),
        LocalState::Conflict => theme.base().fg(theme.error),
    }
}

fn draw_delete_confirmation(frame: &mut Frame<'_>, package: Package, theme: &Theme) {
    let area = centered(frame.area(), 62, 7);
    frame.render_widget(Clear, area);
    let panel = Panel::new("Confirm deletion").focused(true).block(theme);
    let content = panel.inner(area);
    frame.render_widget(panel, area);
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(format!("Delete the local {} repository?", package.name)),
            Line::from("This permanently removes uncommitted and untracked files."),
            Line::from("Press y to delete, or n/Esc to cancel.")
                .style(theme.base().fg(theme.warning)),
        ])
        .style(theme.base())
        .wrap(Wrap { trim: true }),
        content,
    );
}

fn centered(area: Rect, requested_width: u16, requested_height: u16) -> Rect {
    let width = requested_width.min(area.width.saturating_sub(2)).max(1);
    let height = requested_height.min(area.height.saturating_sub(2)).max(1);
    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_wraps_in_both_directions() {
        let mut app = App::new(PathBuf::from("/repos"));
        app.select_previous();
        assert_eq!(app.selected, PACKAGES.len() - 1);
        app.select_next();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn confirmation_rect_stays_inside_small_terminals() {
        let terminal = Rect::new(10, 20, 8, 4);
        let popup = centered(terminal, 62, 7);

        assert!(popup.x >= terminal.x && popup.y >= terminal.y);
        assert!(popup.right() <= terminal.right());
        assert!(popup.bottom() <= terminal.bottom());
    }
}
