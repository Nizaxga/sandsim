use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, read},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use std::io::stdout;

fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    enable_raw_mode()?;

    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, Clear(ClearType::All), MoveTo(10, 5))?;

    println!("2D Sand Simulation. Press 'q' to quit.");

    // Main loop to handle events
    loop {
        if let Event::Key(event) = read()? {
            if event.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
