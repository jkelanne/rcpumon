use psutil::{cpu, sensors, Percent};
use std::{time, error::Error};
use std::io::{Write, stdout};
use std::f64;

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

// In search for a better name
struct App {
    min_width: i32,
    cpu_loads: Vec<Percent>,
    cpu_total: Percent,
    n_logical: i32,
    n_physical: i32,
    collector: cpu::CpuPercentCollector,
}

impl App {
    fn new(min_width: i32, nl: i32, np: i32) -> App {
        let mut ccol: cpu::CpuPercentCollector = cpu::CpuPercentCollector::new().unwrap(); 
        let percpu_percents = ccol.cpu_percent_percpu().unwrap();
        let total_percent = ccol.cpu_percent().unwrap();

        App {
            min_width,
            cpu_loads: percpu_percents,
            cpu_total: total_percent,
            n_logical: nl,
            n_physical: np,
            collector: ccol,
        }
    }

    fn update(&mut self) {
        self.cpu_loads = self.collector.cpu_percent_percpu().unwrap();
        self.cpu_total = self.collector.cpu_percent().unwrap();
    }

    fn get_row_count(&self) -> i32 {
        return ((self.n_logical % self.min_width) + 1)
    }

    fn get_last_column_count(&self) -> i32 {
       return (self.n_logical - (self.min_width * (self.n_logical % self.min_width)))
    }

    fn get_min_width(&self) -> usize {
        return self.min_width as usize;
    }

    fn cpu_percent_as_ratio(&self, cpu_id: usize) -> f64 {
        let tval = self.cpu_loads[cpu_id] as f64 / 100.0;
        if tval < 0.0 {
            return 0.0;
        } else if tval > 1.0 {
            return 1.0;
        } 
        return tval;
    }

    fn cpu_get_name(&self, cpu_id: usize) -> String {
        let s = format!("CPU{}", cpu_id);
        return s;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut collector: cpu::CpuPercentCollector = cpu::CpuPercentCollector::new().unwrap();

    let logical_cpu_count = cpu::cpu_count() as i32;
    let physical_cpu_count = cpu::cpu_count_physical() as i32;

    let temperatures = sensors::temperatures();

    let mut app = App::new(5, logical_cpu_count, physical_cpu_count); 
    terminal_setup()?;

    let backend = TermionBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let borders_style = Borders::ALL;
    //let borders_style = Borders::BOTTOM | Borders::LEFT;
    //
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

        app.update();

        // TODO: MAKE IT DYNAMIC!
        // - Percentage(20) should be calculated
        // Loop through the rows etc..

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                // This chunk contains the rows. so it should be based on app.get_row_count();
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(vec![Constraint::Percentage(20); app.get_row_count() as usize].as_ref())
                .split(f.size());
            // The full rows should be generated here
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(20); app.get_min_width() as usize].as_ref())
                    .split(chunks[0]);
                f.render_widget(get_gauge(&app.cpu_get_name(0), borders_style, app.cpu_percent_as_ratio(0)), chunks[0]);
                f.render_widget(get_gauge(&app.cpu_get_name(1), borders_style, app.cpu_percent_as_ratio(1)), chunks[1]);
                f.render_widget(get_gauge(&app.cpu_get_name(2), borders_style, app.cpu_percent_as_ratio(2)), chunks[2]);
                f.render_widget(get_gauge(&app.cpu_get_name(3), borders_style, app.cpu_percent_as_ratio(3)), chunks[3]);
                f.render_widget(get_gauge(&app.cpu_get_name(4), borders_style, app.cpu_percent_as_ratio(4)), chunks[4]);

            }
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(20); app.get_min_width() as usize].as_ref())
                    .split(chunks[1]);
                f.render_widget(get_gauge(&app.cpu_get_name(5), borders_style, app.cpu_percent_as_ratio(5)), chunks[0]);
                f.render_widget(get_gauge(&app.cpu_get_name(6), borders_style, app.cpu_percent_as_ratio(6)), chunks[1]);
                f.render_widget(get_gauge(&app.cpu_get_name(7), borders_style, app.cpu_percent_as_ratio(7)), chunks[2]);
                f.render_widget(get_gauge(&app.cpu_get_name(8), borders_style, app.cpu_percent_as_ratio(8)), chunks[3]);
                f.render_widget(get_gauge(&app.cpu_get_name(9), borders_style, app.cpu_percent_as_ratio(9)), chunks[4]);

            }
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(20); app.get_min_width() as usize].as_ref())
                    .split(chunks[2]);
                f.render_widget(get_gauge(&app.cpu_get_name(10), borders_style, app.cpu_percent_as_ratio(10)), chunks[0]);
                f.render_widget(get_gauge(&app.cpu_get_name(11), borders_style, app.cpu_percent_as_ratio(11)), chunks[1]);
                f.render_widget(get_gauge(&app.cpu_get_name(12), borders_style, app.cpu_percent_as_ratio(12)), chunks[2]);
                f.render_widget(get_gauge(&app.cpu_get_name(13), borders_style, app.cpu_percent_as_ratio(13)), chunks[3]);
                f.render_widget(get_gauge(&app.cpu_get_name(14), borders_style, app.cpu_percent_as_ratio(14)), chunks[4]);

            }
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(20); app.get_min_width() as usize].as_ref())
                    .split(chunks[3]);
                f.render_widget(get_gauge(&app.cpu_get_name(15), borders_style, app.cpu_percent_as_ratio(15)), chunks[0]);
                f.render_widget(get_gauge(&app.cpu_get_name(16), borders_style, app.cpu_percent_as_ratio(16)), chunks[1]);
                f.render_widget(get_gauge(&app.cpu_get_name(17), borders_style, app.cpu_percent_as_ratio(17)), chunks[2]);
                f.render_widget(get_gauge(&app.cpu_get_name(18), borders_style, app.cpu_percent_as_ratio(18)), chunks[3]);
                f.render_widget(get_gauge(&app.cpu_get_name(19), borders_style, app.cpu_percent_as_ratio(19)), chunks[4]);

            }
            // app.get_last_column_count()
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(20); app.get_min_width() as usize].as_ref())
                    .split(chunks[4]);
                f.render_widget(get_gauge(&app.cpu_get_name(20), borders_style, app.cpu_percent_as_ratio(20)), chunks[0]);
                f.render_widget(get_gauge(&app.cpu_get_name(21), borders_style, app.cpu_percent_as_ratio(21)), chunks[1]);
                f.render_widget(get_gauge(&app.cpu_get_name(22), borders_style, app.cpu_percent_as_ratio(22)), chunks[2]);
                f.render_widget(get_gauge(&app.cpu_get_name(23), borders_style, app.cpu_percent_as_ratio(23)), chunks[3]);
                f.render_widget(get_gauge("AVG", borders_style,  (avg_total / 100.0) as f64), chunks[4]);

            }
        })?;
        
        //thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}
