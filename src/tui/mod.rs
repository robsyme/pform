use std::io;
use std::time::Duration;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::env;

mod app;
mod ui;

pub use app::App;
use crate::SeqeraClient;

pub async fn run() -> Result<()> {
    // Get token from environment
    let token = env::var("TOWER_ACCESS_TOKEN")
        .map_err(|_| anyhow::anyhow!("TOWER_ACCESS_TOKEN environment variable not set"))?;

    // Initialize client
    let client = SeqeraClient::new(token)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new(client);
    app.refresh().await;
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::F(1) => app.show_help(),
                    KeyCode::F(5) => { app.refresh().await; }
                    KeyCode::F(10) => app.toggle_menu(),
                    KeyCode::Tab => app.next_tab(),
                    KeyCode::BackTab => app.previous_tab(),
                    KeyCode::Enter => { app.handle_enter().await; }
                    KeyCode::Left => app.handle_left(),
                    KeyCode::Right => app.handle_right(),
                    KeyCode::Up => app.handle_up(),
                    KeyCode::Down => app.handle_down(),
                    KeyCode::Esc => app.handle_escape(),
                    _ => {}
                }
            }
        }
    }
} 