use psutil::{cpu, sensors};
use std::{time, error::Error};
use std::io::{Write, stdout};
use crossterm::{
    event::{poll, read, Event, KeyCode},
    cursor, 
    terminal,
    execute,
    ErrorKind
};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
    Terminal,
};

//use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

fn get_gauge<'a>(title: &'a str, borders_style: Borders, ratio: f64) -> Gauge<'a> {
    Gauge::default()
        .block(Block::default().title(title).borders(borders_style))
        .style(Style::default().fg(Color::Green))
        .ratio(ratio)
}

fn terminal_setup() -> std::result::Result<(), ErrorKind> {
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, cursor::Hide)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    terminal::enable_raw_mode().unwrap();

    Ok(())
}

fn terminal_cleanup() -> std::result::Result<(), ErrorKind> {
    let mut tmp_stdout = std::io::stdout();
    execute!(tmp_stdout, cursor::MoveTo(0,0))?;
    execute!(tmp_stdout, terminal::Clear(terminal::ClearType::All))?;
    execute!(tmp_stdout, terminal::LeaveAlternateScreen)?;
    execute!(tmp_stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut collector: cpu::CpuPercentCollector = cpu::CpuPercentCollector::new().unwrap();

    let logical_cpu_count = cpu::cpu_count();
    let physical_cpu_count = cpu::cpu_count_physical();

    let temperatures = sensors::temperatures();

    terminal_setup()?;

    let backend = TermionBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let borders_style = Borders::ALL;
    //let borders_style = Borders::BOTTOM | Borders::LEFT;

    loop {
        let avg_percents = collector.cpu_percent_percpu().unwrap();
        let avg_total = collector.cpu_percent().unwrap();
        if poll(time::Duration::from_millis(1000))? {
            let event = read()?;
            if event == Event::Key(KeyCode::Char('q').into()) {
                println!("Exiting!!");
                terminal_cleanup()?;
                
                println!("Logical Units: {}; Physical Units: {}", logical_cpu_count, physical_cpu_count);
                for t in temperatures {
                    let t_unwrapped = t.unwrap();
                    println!("{:?}: {}", t_unwrapped.label(), t_unwrapped.current().celsius());
                }
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

                f.render_widget(get_gauge("CPU0", borders_style, (avg_percents[0] / 100.0) as f64), chunks[0]);
                f.render_widget(get_gauge("CPU1", borders_style, (avg_percents[1] / 100.0) as f64), chunks[1]);
                f.render_widget(get_gauge("CPU2", borders_style, (avg_percents[2] / 100.0) as f64), chunks[2]);
                f.render_widget(get_gauge("CPU3", borders_style, (avg_percents[3] / 100.0) as f64), chunks[3]);
                f.render_widget(get_gauge("CPU4", borders_style, (avg_percents[4] / 100.0) as f64), chunks[4]);

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

                f.render_widget(get_gauge("CPU5", borders_style, (avg_percents[5] / 100.0) as f64), chunks[0]);
                f.render_widget(get_gauge("CPU6", borders_style, (avg_percents[6] / 100.0) as f64), chunks[1]);
                f.render_widget(get_gauge("CPU7", borders_style, (avg_percents[7] / 100.0) as f64), chunks[2]);
                f.render_widget(get_gauge("CPU8", borders_style, (avg_percents[8] / 100.0) as f64), chunks[3]);
                f.render_widget(get_gauge("CPU9", borders_style, (avg_percents[9] / 100.0) as f64), chunks[4]);
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

                f.render_widget(get_gauge("CPU10", borders_style, (avg_percents[10] / 100.0) as f64), chunks[0]);
                f.render_widget(get_gauge("CPU11", borders_style, (avg_percents[11] / 100.0) as f64), chunks[1]);
                f.render_widget(get_gauge("CPU12", borders_style, (avg_percents[12] / 100.0) as f64), chunks[2]);
                f.render_widget(get_gauge("CPU13", borders_style, (avg_percents[13] / 100.0) as f64), chunks[3]);
                f.render_widget(get_gauge("CPU14", borders_style, (avg_percents[14] / 100.0) as f64), chunks[4]);
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

                f.render_widget(get_gauge("CPU15", borders_style, (avg_percents[15] / 100.0) as f64), chunks[0]);
                f.render_widget(get_gauge("CPU16", borders_style, (avg_percents[16] / 100.0) as f64), chunks[1]);
                f.render_widget(get_gauge("CPU17", borders_style, (avg_percents[17] / 100.0) as f64), chunks[2]);
                f.render_widget(get_gauge("CPU18", borders_style, (avg_percents[18] / 100.0) as f64), chunks[3]);
                f.render_widget(get_gauge("CPU19", borders_style, (avg_percents[19] / 100.0) as f64), chunks[4]);
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

                f.render_widget(get_gauge("CPU20", borders_style, (avg_percents[0] / 100.0) as f64), chunks[0]);
                f.render_widget(get_gauge("CPU21", borders_style, (avg_percents[1] / 100.0) as f64), chunks[1]);
                f.render_widget(get_gauge("CPU22", borders_style, (avg_percents[2] / 100.0) as f64), chunks[2]);
                f.render_widget(get_gauge("CPU23", borders_style, (avg_percents[3] / 100.0) as f64), chunks[3]);
                f.render_widget(get_gauge("AVG", borders_style, (avg_total / 100.0) as f64), chunks[4]);
            }
        })?;
        
        //thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
