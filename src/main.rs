use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* Setup
     *
     * - capture stdout in raw mode
     * - create crossterm backend
     * - create terminal object
     * - create App object
     * - TODO: enter app run loop
     */

    let mut stdout = io::stdout();

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = app::App::default();
    let res = app.run(&mut terminal);

    /* Cleanup
     *
     * - return terminal to default state
     * - print app errors (if any)
     * - return
     */
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
