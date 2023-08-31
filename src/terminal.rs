use std::{
    error::Error,
    io::{self, Stdout}, rc::Rc, cell::RefCell,
};

use crossterm::{
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use event_emitter_rs::EventEmitter;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::global::state::State;

mod ui;
mod keying;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, mut state: State, event_emitter: Rc<RefCell<EventEmitter>>) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|f| ui::main(f, &mut state))?;

        if !keying::listen(Rc::clone(&event_emitter))? {
            break;
        }
    })
}

