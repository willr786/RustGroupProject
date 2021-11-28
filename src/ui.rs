use std::io;

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::backend::Backend;
use tui::Terminal;

pub struct UI<'a, B: Backend> {
    terminal: Terminal<B>,
    history: Vec<ListItem<'a>>,
}

impl<'a, B: Backend> UI<'a, B> {
    pub fn new(backend: B) -> io::Result<UI<'a, B>> {
        Ok(UI {
            terminal: Terminal::new(backend)?,
            history: Vec::<ListItem>::new(),
        })
    }

    pub fn add_history(&mut self, entry: String) {
        self.history.push(ListItem::new(entry));
    }

    pub fn render(&mut self, input: String, result: String) {
        self.terminal.clear();

        let history_list = self.history.clone();

        self.terminal.draw(move |f| {
            // Create Layout Chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(f.size());

            let top_chunk = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[0]);

            let left_chunk = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(0),
                        Constraint::Percentage(85),
                        Constraint::Percentage(15),
                    ]
                    .as_ref(),
                )
                .split(top_chunk[0]);

            //Create and Render Widgets
            //Input Block
            let block = Block::default().title("Input").borders(Borders::ALL);
            let input_inside = Paragraph::new(input)
                .block(block)
                .alignment(Alignment::Left);
            f.render_widget(input_inside, chunks[1]);

            //History Block
            let block = Block::default().title("History").borders(Borders::ALL);
            let items = history_list;
            let list = List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>")
                .block(block);
            f.render_widget(list, left_chunk[1]);

            // Result Block
            let block = Block::default().title("Result").borders(Borders::ALL);
            let result_inside = Paragraph::new(result)
                .block(block)
                .alignment(Alignment::Left);
            f.render_widget(result_inside, left_chunk[2]);

            // Info Block
            let block = Block::default().title("Info").borders(Borders::ALL);
            let text = vec![
                Spans::from(vec![Span::raw("Project Info"), Span::raw(".")]),
                Spans::from("More Info"),
            ];
            let info = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            f.render_widget(info, top_chunk[1]);
        }).expect("draw to terminal");
    }
}
