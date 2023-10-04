use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
};
use error::VimaError;
use ima::VisualIMA;
use ima_core::{IMA, ImaOptions, parse_debug};
use ratatui::prelude::*;

mod io;
mod ima;
mod error;

fn main() {
    let result = run();
    match result {
        Ok(_) => {/* all good */},
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<(), VimaError> {
    // setup ima
    let ima_options = ImaOptions::new(std::env::args())?;
    let file = match std::fs::read_to_string(&ima_options.file) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };

    let program = parse_debug(&file)?;
    let ima = IMA::new(program, ima_options);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // setup visual ima
    let visual_ima = VisualIMA::new(ima, &mut terminal);

    // run
    let res = visual_ima.run();

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}
