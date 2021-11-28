use std::io;
use tui::backend::CrosstermBackend;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* TODO: Setup input loop */

    /* Setup render loop:
     *
     * - capture stdout
     * - create crossterm backend
     * - create UI object
     * - TODO: enter render loop at set tickrate
     */

    let stdout = io::stdout();
    let ui_backend = CrosstermBackend::new(stdout);
    let mut ui = ui::UI::new(ui_backend).expect("ui could not be initialized");

    ui.add_history("test1".to_string());
    ui.add_history("test2".to_string());
    ui.add_history("test3".to_string());

    ui.render("test".to_string(), "test".to_string());

    Ok(())
}
