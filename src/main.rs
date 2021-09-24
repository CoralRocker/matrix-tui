#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::io;
use std::fmt::Write;
use termion::raw::IntoRawMode;
use tui::{Terminal, backend};
use termion::event::Key;
use tui::layout::*;
use std::{time, thread};

use matrix::InputHandler;
use matrix::MatrixWidget;

fn main() -> Result<(), io::Error>{
    
    let stdout = io::stdout().into_raw_mode()?;
    let backend = backend::TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = InputHandler::AsyncIO::new();
    let mut errorLog = String::new();

    terminal.clear()?;

    let mut matrix = MatrixWidget::MatrixWidget::new(terminal.size()?);
    matrix.populate();

    let mut exitLoop = false;
    while !exitLoop {
        
        match input.get()? {
            Key::Char('q') => exitLoop = true,
            _ => ()
        }
        
        {
            let os = terminal.size()?;
            terminal.autoresize()?;
            let ns = terminal.size()?;

            if os.width != ns.width || os.height != ns.height {
                writeln!(errorLog, "Screen Resized: Old Size: {} {}, New Size; {} {}", os.width, os.height, ns.width, ns.height);
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(100)
                    ].as_ref()
                    )
                .split(f.size());
            f.render_widget(matrix.clone(), chunks[0]);

        })?;
        matrix.age();
        matrix.populate();
        thread::sleep(time::Duration::from_millis(50));
    }
    
    terminal.clear()?;
    println!("{}", errorLog);

    Ok(())
}
