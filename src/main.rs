use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect, Alignment};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Row, Table, Widget, Paragraph, Wrap};
use tui::text::{Spans, Span};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create Terminal GUI
    terminal.clear();
    terminal.draw(|f| {
        // Create Layout Chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(f.size());

        let top_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(chunks[0]);
        
        let left_chunk = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Percentage(0), Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(top_chunk[0]);

        //Create and Render Widgets
        //Input Block
        let block = Block::default().title("Input").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);

        //History Block
        let block = Block::default().title("History").borders(Borders::ALL);
        let items = [
            ListItem::new("Item 1"),
            ListItem::new("Item 2"),
            ListItem::new("Item 3"),
        ];
        let list = List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .block(block);
        f.render_widget(list, left_chunk[1]);

        // Result Block
        let block = Block::default().title("Result").borders(Borders::ALL);
        f.render_widget(block, left_chunk[2]);
        
        // Info Block
        let block = Block::default().title("Info").borders(Borders::ALL);
        let text = vec![
            Spans::from(vec![
                Span::raw("Project Info"),
                Span::raw("."),
            ]),
            Spans::from("More Info"),
        ];
        let info = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(info, top_chunk[1]);

    });
    Ok(())
}
