#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::io;
use std::fmt::Write;
use termion::raw::IntoRawMode;
use tui::{Terminal, backend};
use termion::event::Key;
use tui::layout::*;
use std::{time, thread};
use tui::widgets::*;

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
    let mut splitScreen = false;
    while !exitLoop {
        
        match input.get()? {
            Key::Char('q') => exitLoop = true,
            Key::Char('s') => splitScreen = if splitScreen { false } else { true },
            _ => ()
        }
        
        {
            let os = terminal.size()?;
            terminal.autoresize()?;
            let ns = terminal.size()?;

            if os.width != ns.width || os.height != ns.height {
                if let Err(e) = writeln!(errorLog, "Screen Resized: Old Size: {} {}, New Size; {} {}", os.width, os.height, ns.width, ns.height) {
                    println!("Oopsie Woopsie somebody made a big ol fucky wucky and couldn't write to the error logfile. ");
                    println!("Err: {}", e);
                };
            }
        }


        terminal.draw(|f| {
            
            let constrs = if splitScreen {
                [ Constraint::Percentage(50), Constraint::Percentage(50) ]
            }else{
                [ Constraint::Percentage(100), Constraint::Percentage(0) ]
            };
            
            let block = Block::default()
                .title("Test Block")
                .borders(Borders::ALL);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    constrs.as_ref()
                    )
                .split(f.size());
            f.render_widget(matrix.clone(), chunks[0]);
            f.render_widget(block, chunks[1]);
        })?;
        matrix.age();
        matrix.populate();
        thread::sleep(time::Duration::from_millis(50));
    }
    
    terminal.clear()?;
    
    

    println!("{}", errorLog);

    Ok(())
}
