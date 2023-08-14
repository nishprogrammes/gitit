mod ui;
mod keying;

use std::{
    error::Error,
    io::{self, Stdout},
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

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crate::cache::Cache;

use crate::controller::state::State;

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

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, state: &mut State, cache: &mut Cache) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|f| ui::main(f, state, cache))?;

        if !keying::listen(state)? {
            break;
        }
    })
}

