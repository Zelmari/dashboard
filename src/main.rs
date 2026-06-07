mod app;
mod system;
mod theme;
mod ui;

use std::io;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;

const DEFAULT_REFRESH_MS: u64 = 5_000;

fn main() -> Result<()> {
    let refresh_ms = parse_refresh_arg().unwrap_or(DEFAULT_REFRESH_MS);

    let mut app = App::new(refresh_ms)
        .context("Failed to initialise system collector. Do you have permission to read /proc?")?;

    let mut terminal = setup_terminal().context("Failed to set up terminal")?;

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(info);
    }));

    let result = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal).context("Failed to restore terminal")?;

    result.context("Event loop exited with an error")?;

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let tick_duration = Duration::from_millis(app.refresh_ms);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let elapsed = last_tick.elapsed();
        let timeout = tick_duration.saturating_sub(elapsed);

        if event::poll(timeout).context("Failed to poll for events")? {
            match event::read().context("Failed to read event")? {
                Event::Key(key) => {
                    if key.modifiers.contains(KeyModifiers::CONTROL)
                        && key.code == KeyCode::Char('c')
                    {
                        app.should_quit = true;
                    } else {
                        app.on_key(key.code);
                    }
                }
                Event::Resize(_, _) => {}
                Event::Mouse(_) => {}
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_duration {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).context("Failed to create terminal")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;
    Ok(())
}

fn parse_refresh_arg() -> Option<u64> {
    let args: Vec<String> = std::env::args().collect();
    let pos = args.iter().position(|a| a == "--refresh-ms")?;
    args.get(pos + 1)?.parse().ok()
}
