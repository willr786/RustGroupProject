use std::io;
mod ui;

fn main() {
    let mut ui = ui::UI::new(io::stdout()).expect("ui could not be inizilized");
    // TODO: input system

    ui.add_history("test1".to_string());
    ui.add_history("test2".to_string());
    ui.add_history("test3".to_string());

    ui.render("test".to_string(), "test".to_string());
}
