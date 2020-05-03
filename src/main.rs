use psutil::{cpu, Percent};
use std::{thread, time, error::Error};
use std::io::{Write, stdout};
use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    ExecutableCommand, 
    cursor, 
    terminal,
    execute,
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Widget},
    Terminal,
};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

fn setup_terminal() -> std::io::Result<()> {
    Ok(())
}

fn get_gauge<'a>(title: &'a str, borders_style: Borders, ratio: f64) -> Gauge<'a> {
    Gauge::default()
        .block(Block::default().title(title).borders(borders_style))
        .style(Style::default().fg(Color::Green))
        //.ratio((avg_percents[0] / 100.0) as f64);
        .ratio(ratio)
    
//    gauge;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut collector: cpu::CpuPercentCollector = cpu::CpuPercentCollector::new().unwrap();
    let mut avg_percents: Vec<Percent> = Vec::new();

    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let borders_style = Borders::ALL;
    //let borders_style = Borders::BOTTOM | Borders::LEFT;

    loop {
        avg_percents = collector.cpu_percent_percpu().unwrap();
        let avg_total = collector.cpu_percent().unwrap();
        if poll(time::Duration::from_millis(100))? {
            let event = read()?;
            if event == Event::Key(KeyCode::Char('q').into()) {
                println!("Exiting!!");
                terminal.show_cursor()?;

                terminal::disable_raw_mode().unwrap();
                break;
            }
        }

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    //.margin(1)
                    .constraints([
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                        ].as_ref())
                    .split(chunks[0]);

/*                let gauge = Gauge::default()
                    .block(Block::default().title("CPU0").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[0] / 100.0) as f64);*/
                //f.render_widget(gauge, chunks[0]);
                f.render_widget(get_gauge("CPU0", borders_style, (avg_percents[0] / 100.0) as f64), chunks[0]);

                let gauge = Gauge::default()
                    .block(Block::default().title("CPU1").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[1] / 100.0) as f64);
                f.render_widget(gauge, chunks[1]);                
                
                let gauge = Gauge::default()
                    .block(Block::default().title("CPU2").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[2] / 100.0) as f64);
                f.render_widget(gauge, chunks[2]);                
                
                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU3").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[3] / 100.0) as f64);
                f.render_widget(gauge, chunks[3]);

                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU4").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[4] / 100.0) as f64);
                f.render_widget(gauge, chunks[4]);

            }
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    //.margin(1)
                    .constraints([
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                        ].as_ref())
                    .split(chunks[1]);

                let gauge = Gauge::default()
                    .block(Block::default().title("CPU5").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[5] / 100.0) as f64);
                f.render_widget(gauge, chunks[0]);

                let gauge = Gauge::default()
                    .block(Block::default().title("CPU6").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[6] / 100.0) as f64);
                f.render_widget(gauge, chunks[1]);                
                
                let gauge = Gauge::default()
                    .block(Block::default().title("CPU7").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[7] / 100.0) as f64);
                f.render_widget(gauge, chunks[2]);                
                
                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU8").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[8] / 100.0) as f64);
                f.render_widget(gauge, chunks[3]);

                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU9").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[9] / 100.0) as f64);
                f.render_widget(gauge, chunks[4]);

            }
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20),
                            Constraint::Percentage(20), 
                        ].as_ref())
                    .split(chunks[2]);

                let gauge = Gauge::default()
                    .block(Block::default().title("CPU10").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[10] / 100.0) as f64);
                f.render_widget(gauge, chunks[0]);

                let gauge = Gauge::default()
                    .block(Block::default().title("CPU11").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[11] / 100.0) as f64);
                f.render_widget(gauge, chunks[1]);                
                
                let gauge = Gauge::default()
                    .block(Block::default().title("CPU12").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[12] / 100.0) as f64);
                f.render_widget(gauge, chunks[2]);                
                
                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU13").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[13] / 100.0) as f64);
                f.render_widget(gauge, chunks[3]);
                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU14").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[14] / 100.0) as f64);
                f.render_widget(gauge, chunks[4]);

            }
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20),
                            Constraint::Percentage(20),
                        ].as_ref())
                    .split(chunks[3]);

                let gauge = Gauge::default()
                    .block(Block::default().title("CPU15").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[15] / 100.0) as f64);
                f.render_widget(gauge, chunks[0]);

                let gauge = Gauge::default()
                    .block(Block::default().title("CPU16").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[16] / 100.0) as f64);
                f.render_widget(gauge, chunks[1]);                
                
                let gauge = Gauge::default()
                    .block(Block::default().title("CPU17").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[17] / 100.0) as f64);
                f.render_widget(gauge, chunks[2]);                
                
                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU18").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[18] / 100.0) as f64);
                f.render_widget(gauge, chunks[3]);                

                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU19").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[19] / 100.0) as f64);
                f.render_widget(gauge, chunks[4]);

            }
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20), 
                            Constraint::Percentage(20),
                            Constraint::Percentage(20), 
                        ].as_ref())
                    .split(chunks[4]);

                let gauge = Gauge::default()
                    .block(Block::default().title("CPU20").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[20] / 100.0) as f64);
                f.render_widget(gauge, chunks[0]);

                let gauge = Gauge::default()
                    .block(Block::default().title("CPU21").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[21] / 100.0) as f64);
                f.render_widget(gauge, chunks[1]);

                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU22").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[22] / 100.0) as f64);
                f.render_widget(gauge, chunks[2]);

                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU23").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_percents[23] / 100.0) as f64);
                f.render_widget(gauge, chunks[3]);

                let gauge = Gauge::default()                    
                    .block(Block::default().title("AVG").borders(borders_style))
                    .style(Style::default().fg(Color::Green))
                    .ratio((avg_total / 100.0) as f64);
                f.render_widget(gauge, chunks[4]);


/*                
                let gauge = Gauge::default()
                    .block(Block::default().title("CPU6").borders(Borders::ALL))
                    .style(Style::default().fg(Color::Green))
                    .ratio(0.33);
                f.render_widget(gauge, chunks[2]);                
                
                let gauge = Gauge::default()                    
                    .block(Block::default().title("CPU7").borders(Borders::ALL))
                    .style(Style::default().fg(Color::Green))
                    .ratio(0.33);
                f.render_widget(gauge, chunks[3]);
*/
            }

        })?;
        
        thread::sleep(time::Duration::from_secs(1));
/*        if let Err(e) = check_events() {
            println!("Error: {:?}", e);
        }*/
    }
//        stdout.execute(cursor::MoveTo(0,0));
//        stdout.execute(terminal::Clear(terminal::ClearType::All));


//        for ac in avg_percents {
//            println!("{:.1}%", ac);
//
//        }
       
//        stdout.flush();
    Ok(())
}
