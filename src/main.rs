use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind, poll, read},
    execute, queue,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use std::io::{Write, stdout};
use std::time::{Duration, Instant};

const TICK: Duration = Duration::from_millis(30);
const WIDTH: usize = 60;
const HEIGHT: usize = 24;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Sand,
}

type Grid = Vec<Vec<Cell>>;

fn draw_frame(stdout: &mut std::io::Stdout) -> Result<(), std::io::Error> {
    // top row
    execute!(stdout, MoveTo(0, 0))?;
    write!(stdout, "+")?;
    for _ in 0..WIDTH {
        write!(stdout, "-")?;
    }
    write!(stdout, "+")?;
    // parallel columns
    for y in 1..HEIGHT + 1 {
        execute!(stdout, MoveTo(0, y as u16))?;
        write!(stdout, "|")?;
        execute!(stdout, MoveTo((WIDTH + 1) as u16, y as u16))?;
        write!(stdout, "|")?;
    }

    // bottom row
    execute!(stdout, MoveTo(0, (HEIGHT + 1) as u16))?;
    write!(stdout, "+")?;
    for _ in 0..WIDTH {
        write!(stdout, "-")?;
    }
    write!(stdout, "+")?;
    Ok(())
}

fn sand_fall(grid: &mut Grid) {
    for y in (0..HEIGHT - 1).rev() {
        for x in 0..WIDTH {
            if grid[y][x] != Cell::Sand {
                continue;
            }

            if grid[y + 1][x] == Cell::Empty {
                grid[y][x] = Cell::Empty;
                grid[y + 1][x] = Cell::Sand;
            } else if x > 1 && grid[y + 1][x - 1] == Cell::Empty {
                grid[y][x] = Cell::Empty;
                grid[y + 1][x - 1] = Cell::Sand;
            } else if x + 1 < WIDTH && grid[y + 1][x + 1] == Cell::Empty {
                grid[y + 1][x + 1] = Cell::Sand;
                grid[y][x] = Cell::Empty;
            }
        }
    }
}

fn render(grid: &Grid, stdout: &mut std::io::Stdout) -> Result<(), std::io::Error> {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            execute!(stdout, MoveTo((x + 1) as u16, (y + 1) as u16))?;
            let ch = match grid[y][x] {
                Cell::Empty => ' ',
                Cell::Sand => '*',
            };
            queue!(stdout, MoveTo((x + 1) as u16, (y + 1) as u16))?;
            write!(stdout, "{}", ch)?;
        }
    }
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    let mut grid = vec![vec![Cell::Empty; WIDTH]; HEIGHT];
    let mut last_update = Instant::now();
    let mut mouse_state = false;
    let mut mouse_pos = (0, 0);
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture,)?;

    // Main loop to handle events
    let result: Result<(), Box<dyn std::error::Error>> = (|| {
        execute!(stdout, Clear(ClearType::All))?;
        draw_frame(&mut stdout)?;
        execute!(stdout, MoveTo(0, (HEIGHT + 3) as u16))?;
        println!("Sand Simulation! Click to place sand. Press 'q' to quit. Press 'r' to reset.");
        stdout.flush()?;
        loop {
            sand_fall(&mut grid);
            if poll(Duration::from_millis(30))? {
                match read()? {
                    Event::Key(event) => {
                        if event.code == KeyCode::Char('q') { break; }
                        if event.code == KeyCode::Char('r') { grid = vec![vec![Cell::Empty; WIDTH]; HEIGHT]; }
                    } 
                    Event::Mouse(mouse_event) => {
                        mouse_pos = (mouse_event.column as usize, mouse_event.row as usize);
                        match mouse_event.kind {
                            MouseEventKind::Down(_) => {
                                mouse_state = true;
                            }
                            MouseEventKind::Up(_) => {
                                mouse_state = false;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            if mouse_state {
                let (x, y) = mouse_pos;
                if (1..=WIDTH).contains(&x) && (1..=HEIGHT).contains(&y) {
                    grid[y - 1][x - 1] = Cell::Sand;
                }
            }

            if last_update.elapsed() >= TICK {
                last_update = Instant::now();
                sand_fall(&mut grid);
                render(&grid, &mut stdout)?;
            }
        }

        Ok(())
    })();

    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    result
}
