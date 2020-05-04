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

/***
 * TODO:
 *  - Add functionality for testing different sizes of grids (for different number of CPUs)
 *  - Add *themes* or something
 *  - Add Colors for low, med and high loads
 *  - Add option for displaying current CPU freqs
 *  - Add option for displaying CPU thermal level
 */

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
    terminal::enable_raw_mode()?;

    Ok(())
}

fn terminal_cleanup() -> std::result::Result<(), ErrorKind> {
    let mut tmp_stdout = std::io::stdout();
    execute!(tmp_stdout, cursor::MoveTo(0,0))?;
    execute!(tmp_stdout, terminal::Clear(terminal::ClearType::All))?;
    execute!(tmp_stdout, terminal::LeaveAlternateScreen)?;
    execute!(tmp_stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;

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
    fn new(min_width: i32) -> App {
        let mut ccol: cpu::CpuPercentCollector = cpu::CpuPercentCollector::new().unwrap(); 
        let percpu_percents = ccol.cpu_percent_percpu().unwrap();
        let total_percent = ccol.cpu_percent().unwrap();

        let logical_cpu_count = cpu::cpu_count() as i32;
        let physical_cpu_count = cpu::cpu_count_physical() as i32;

        App {
            min_width,
            cpu_loads: percpu_percents,
            cpu_total: total_percent,
            n_logical: logical_cpu_count,
            n_physical: physical_cpu_count,
            collector: ccol,
        }
    }

    fn update(&mut self) {
        self.cpu_loads = self.collector.cpu_percent_percpu().unwrap();
        self.cpu_total = self.collector.cpu_percent().unwrap();
    }

    fn get_row_count(&self) -> usize {
        return (self.n_logical % self.min_width) as usize + 1
    }

    /* Do we need this? */
    fn get_last_column_count(&self) -> i32 {
       return self.n_logical - (self.min_width * (self.n_logical % self.min_width))
    }

    /* Returs the number of elements in a row */
    fn get_width(&self) -> usize {
        return self.min_width as usize;
    }

    fn cpu_percent_as_ratio(&self, cpu_id: usize) -> f64 {
        if cpu_id < self.cpu_loads.len() {
            let tval = self.cpu_loads[cpu_id] as f64 / 100.0;
            if tval < 0.0 {
                return 0.0;
            } else if tval > 1.0 {
                return 1.0;
            }

            return tval;
        }
        return 0.0;
    }

    fn cpu_get_name(&self, cpu_id: usize) -> String {
        let s = format!("CPU{}", cpu_id);
        return s;
    }

    fn is_valid_cpu_index(&self, index: usize) -> bool {
        if index < self.cpu_loads.len() {
            return true;
        }
        return false;
    }

    fn get_avg_total(&self) -> Percent {
        return self.cpu_total;
    }

    fn logical_cpu_count(&self) -> i32 {
        return self.n_logical;
    }

    fn physical_cpu_count(&self) -> i32 {
        return self.n_physical;
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    let temperatures = sensors::temperatures();

    let mut app = App::new(5); 
    terminal_setup()?;

    let backend = TermionBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let borders_style = Borders::ALL;
    let cell_width_percent: u16 = 100 / app.get_width() as u16;
    let row_height_percent: u16 = 100 / app.get_row_count() as u16;
    
    loop {
        let avg_total = app.get_avg_total();
        if poll(time::Duration::from_millis(1000))? {
            let event = read()?;
            if event == Event::Key(KeyCode::Char('q').into()) {
                println!("Exiting!!");
                terminal_cleanup()?;
                
                // Just for testing.. if we want to display the thermal reading at some point.
                println!("Logical Units: {}; Physical Units: {}", app.logical_cpu_count(), app.physical_cpu_count());
                for t in temperatures {
                    let t_unwrapped = t.unwrap();
                    println!("{:?}: {}", t_unwrapped.label(), t_unwrapped.current().celsius());
                }
                break;
            }
        }

        app.update();


        terminal.draw(|mut f| {
            // Split the view in vertical chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(vec![Constraint::Percentage(row_height_percent); app.get_row_count() as usize].as_ref())
                .split(f.size());
            // Fill the vertical chunks with gauges
            for r in 0..app.get_row_count() {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(cell_width_percent); app.get_width() as usize].as_ref())
                    .split(chunks[r]);
                for n in 0..=(app.get_width() - 1) {
                    let i = (r * 5) + n;
                    if app.is_valid_cpu_index(i) {
                        f.render_widget(get_gauge(&app.cpu_get_name(i), borders_style, app.cpu_percent_as_ratio(i)), chunks[n]);
                    } else {
                        f.render_widget(get_gauge("AVG", borders_style,  (avg_total / 100.0) as f64), chunks[n]);
                    }
                }
            }
        })?;
    }
    Ok(())
}
