use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::{Frame, Terminal};

pub struct App {
    input: String,
    history: Vec<String>,
}
impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            history: Vec::new(),
        }
    }
}
impl App {
    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        // TODO: remove
        self.history.push("test1".to_string());
        self.history.push("test2".to_string());
        self.history.push("test3".to_string());

        loop {
            let calculation = App::calculate(&self.input);

            terminal
                .draw(|f| self.ui(f, &calculation))
                .expect("to draw on terminal");

            match event::read().expect("to read events") {
                // on Enter
                Event::Key(KeyEvent {
                    modifiers: _,
                    code: KeyCode::Enter,
                }) => {
                    self.history.push("TODO: History".to_string());
                    self.input = String::new();
                }

                Event::Key(KeyEvent {
                    modifiers: _,
                    code: KeyCode::Backspace,
                }) => {
                    self.input.pop();
                }

                // control + c
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('c'),
                }) => {
                    break Ok(());
                }

                // normal char
                Event::Key(KeyEvent {
                    modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                    code: KeyCode::Char(c),
                }) => {
                    self.input.push(c);
                }

                _ => {}
            }
        }
    }

    fn calculate(input: &str) -> Option<String> {
        match sscanf::scanf!(input, "{} {} {}", f64, char, f64) {
            Some((x, '+', y)) => Some(format!("{:04}", x + y)),
            Some((x, '-', y)) => Some(format!("{:04}", x - y)),
            Some((x, '*', y)) => Some(format!("{:04}", x * y)),
            Some((x, '/', y)) => Some(format!("{:04}", x / y)),
            _ => None,
        }
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>, calculation: &Option<String>) {
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
        let input_inside = Paragraph::new(self.input.clone())
            .block(block)
            .alignment(Alignment::Left);
        f.render_widget(input_inside, chunks[1]);

        //History Block
        let block = Block::default().title("History").borders(Borders::ALL);
        let items: Vec<ListItem> = self
            .history
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
                ListItem::new(content)
            })
            .collect();
        let list = List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .block(block);
        f.render_widget(list, left_chunk[1]);

        // Result Block
        let block = Block::default().title("Result").borders(Borders::ALL);
        let prediction = match calculation {
            Some(prediction) => prediction,
            None => "Could not predict...",
        };
        let result_inside = Paragraph::new(prediction)
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
    }
}
