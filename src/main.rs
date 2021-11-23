use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
mod ui;

fn main() {

    let mut ui = ui::UI::new(io::stdout()).expect("ui could not be inizilized");
    
    let mut history = vec!["test1".to_string(), "test2".to_string(), "test3".to_string()];
    ui.render("test".to_string(),"test".to_string());

}
