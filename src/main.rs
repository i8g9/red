use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal, Frame,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    error::Error,
//    fs::DirEntry,
    io,
    io::{stdout, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode();
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

    terminal.show_cursor()?;
    
    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {

        terminal.draw(|f| {
            draw_ui(f);
            draw_command_bar(f);
        })?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn draw_ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = Layout::default()
       .direction(Direction::Vertical)
       .margin(0)
       .constraints(
               [
                   Constraint::Length(1),
                   Constraint::Percentage(85),
                   Constraint::Percentage(15),
                   Constraint::Length(1),
               ]
               .as_ref()
           )
       .split(f.size());

   let parentdirectory_files_preview_chunks = Layout::default()
       .direction(Direction::Horizontal)
       .margin(0)
       .constraints(
               [
                   Constraint::Ratio(2, 10),
                   Constraint::Ratio(5, 10),
                   Constraint::Ratio(3, 10),
               ]
               .as_ref()
           )
       .split(chunks[1]);

    let command_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
                [
                    Constraint::Percentage(100),
                ]
                .as_ref()
            )
        .split(chunks[2]);

   let parentdirectory_block = Block::default()
       .title("Parent Directory")
       .borders(Borders::ALL)
       .style(Style::default().fg(Color::White).bg(Color::Black));

   let files_block = Block::default()
       .title("Files")
       .borders(Borders::ALL)
       .style(Style::default().fg(Color::White).bg(Color::Black));

   let preview_block = Block::default()
       .title("Preview")
       .borders(Borders::ALL)
       .style(Style::default().fg(Color::White).bg(Color::Black));

   let command_block = Block::default()
       .title("Command Bar")
       .borders(Borders::ALL)
       .style(Style::default().fg(Color::White).bg(Color::Black));
    

   f.render_widget(parentdirectory_block,  parentdirectory_files_preview_chunks[0]);
   f.render_widget(files_block,   parentdirectory_files_preview_chunks[1]);
   f.render_widget(preview_block, parentdirectory_files_preview_chunks[2]);
   f.render_widget(command_block, command_chunks[0]);


}

fn draw_command_bar<B: Backend>(f: &mut Frame<B>) {
    
    
    
    
    
    
    
    
    
    

    
    
    
    
    
    
}



