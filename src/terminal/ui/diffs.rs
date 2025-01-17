use ratatui::{
    backend::Backend,
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::domain;
use crate::global::{
    cursor::Section,
    state::State,
};
use crate::terminal::ui::Render;

pub struct Diff;

impl Render for Diff {
    fn render<B: Backend>(frame: &mut Frame<B>, area: Rect, state: &mut State) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if state.cursor_mut().is_in(&Section::Diff) {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            })
            .padding(Padding::uniform(1))
            .title("Changes");

        let paragraph = generate_git_paragaph(block, state);

        frame.render_widget(paragraph, area);
    }
}


pub fn generate_git_paragaph<'a>(
    block: Block<'a>,
    state: &'a mut State,
) -> Paragraph<'a> {
    let m_file = state.get_current_file();
    match m_file {
        None => Paragraph::new("No file available").block(block),
        Some(m_file) => {
            if state.cursor().is_in(&Section::Files) || state.cursor().is_in(&Section::Diff) {
                let binding = domain::get_diff_string(m_file);
                let diff = binding.lines();
                let mut text: Vec<Line> = vec![];

                for line in diff.zip(0..) {
                    if line.1 < 4 || line.0.starts_with("+++") {
                        text.push(Line::from(Span::styled(
                            String::from(line.0),
                            Style::default().add_modifier(Modifier::BOLD),
                        )));
                        continue;
                    }

                    if line.0.starts_with("@@") {
                        let end_ref = line
                            .0
                            .rfind("@@")
                            .expect("Git diff returned wrong formating");
                        text.push(Line::from(vec![
                            Span::styled(
                                String::from(&line.0[..(end_ref + 2)]),
                                Style::default().fg(Color::LightBlue),
                            ),
                            Span::raw(String::from(&line.0[(end_ref + 2)..])),
                        ]));
                        continue;
                    }

                    if line.0.starts_with("+") {
                        text.push(Line::from(Span::styled(
                            String::from(line.0),
                            Style::default().fg(Color::Green),
                        )));
                        continue;
                    }

                    if line.0.starts_with("-") {
                        text.push(Line::from(Span::styled(
                            String::from(line.0),
                            Style::default().fg(Color::Red),
                        )));
                        continue;
                    }

                    text.push(Line::from(Span::styled(
                        String::from(line.0),
                        Style::default(),
                    )));
                }

                return Paragraph::new(text)
                    .block(block)
                    .scroll(state.cursor_mut().get_diff_scroll());
            }

            Paragraph::new("No file selected").block(block)
        }
    }
}
