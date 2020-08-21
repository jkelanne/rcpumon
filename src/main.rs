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

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short = "w", long, default_value = "5")]
    width: usize,

    #[structopt(short = "c", long, default_value = "0")]
    sim_core_count: usize,

    #[structopt(short = "C", long, default_value = "CPUTIN")]
    cputin: String,

    #[structopt(short = "S", long, default_value = "SYSTIN")]
    systin: String,

    #[structopt(short = "T", long)]
    display_temperature: bool,
}

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
    min_width: usize,
    cpu_loads: Vec<Percent>,
    cpu_total: Percent,
    n_logical: usize,
    n_physical: usize,
    display_temperature: bool,
    cpu_temp: f64,
    cpu_max_temp: f64,
    collector: cpu::CpuPercentCollector,
}

impl App {
    fn new(min_width: usize, l_core_count: usize) -> App {
        let mut ccol: cpu::CpuPercentCollector = cpu::CpuPercentCollector::new().unwrap(); 
        let percpu_percents = ccol.cpu_percent_percpu().unwrap();
        let total_percent = ccol.cpu_percent().unwrap();

        let mut logical_cpu_count = cpu::cpu_count() as usize;
        let physical_cpu_count = cpu::cpu_count_physical() as usize;

        if l_core_count > 0 {
            // Limit the number of cores displayer. Mainly used for testing..
            logical_cpu_count = l_core_count;
        }            
        
        App {
            min_width,
            cpu_loads: percpu_percents,
            cpu_total: total_percent,
            n_logical: logical_cpu_count,
            n_physical: physical_cpu_count,
            display_temperature: false,
            cpu_temp: 0.0,
            cpu_max_temp: 100.0,    
            collector: ccol,
        }
    }

    fn update(&mut self) {
        self.cpu_loads = self.collector.cpu_percent_percpu().unwrap();
        self.cpu_total = self.collector.cpu_percent().unwrap();
    }

    fn get_row_count(&self) -> usize {
        if self.n_logical < self.min_width {
            return 1;
        }

        let rval = (self.n_logical / self.min_width) as usize + 1;
        return rval;
    }

    /* Returs the number of elements in a row */
    fn get_width(&self) -> usize {
        return self.min_width;
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
        if index < self.cpu_loads.len() && index < self.n_logical {
            return true;
        }
        return false;
    }

    fn get_avg_total(&self) -> Percent {
        return self.cpu_total;
    }

    fn logical_cpu_count(&self) -> usize {
        return self.n_logical;
    }

    fn physical_cpu_count(&self) -> usize {
        return self.n_physical;
    }

    fn get_cpu_temp(&self) -> f64 {
        return 0.5;
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    // When passing arguments via cargo, use --
    //      `cargo run -- --help`
    //let args: Vec<String> = env::args().collect();
    let opts = Opt::from_args();


    let temperatures = sensors::temperatures();

    let mut app = App::new(opts.width, opts.sim_core_count); 
    terminal_setup()?;

    let backend = TermionBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let borders_style = Borders::ALL;

    // Can't we do this so that row_count isn't mutable? it doesn't have to change after this
    // ever..
    let mut row_count = app.get_row_count();
    if opts.display_temperature == true {
        row_count = row_count + 1;
    }

    let cell_width_percent: u16 = 100 / app.get_width() as u16;
    let row_height_percent: u16 = 100 / row_count as u16;

    loop {
        let avg_total = app.get_avg_total();
        if poll(time::Duration::from_millis(1000))? {
            let event = read()?;
            if event == Event::Key(KeyCode::Char('q').into()) {
                println!("Exiting!!");
                terminal_cleanup()?;
               
                if opts.debug == true {
                    // Just for testing.. if we want to display the thermal reading at some point.
                    println!("Logical Units: {}; Physical Units: {}", app.logical_cpu_count(), app.physical_cpu_count());
                    for t in temperatures {
                        let t_unwrapped = t.unwrap();
                        println!("{:?}: {}", t_unwrapped.label(), t_unwrapped.current().celsius());
                    }
    
                    println!("{:#?}", opts);
    
                    println!("Cell width percentage: {} ({}); Row height percentage: {} ({})", cell_width_percent, 
                        app.get_width(),
                        row_height_percent,
                        app.get_row_count());
                    println!("app.logical_cpu_count(): {}", app.logical_cpu_count());
                    println!("app.get_width(): {}", app.get_width());
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
                .constraints(vec![Constraint::Percentage(row_height_percent); row_count as usize].as_ref())
                .split(f.size());
            // Fill the vertical chunks with gauges
            for r in 0..app.get_row_count() {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(cell_width_percent); app.get_width() as usize].as_ref())
                    .split(chunks[r]);
                for n in 0..=(app.get_width() - 1) {
                    let i = (r * opts.width) + n;
                    if app.is_valid_cpu_index(i) {
                        f.render_widget(get_gauge(&app.cpu_get_name(i), borders_style, app.cpu_percent_as_ratio(i)), chunks[n]);
                    } else {
                        f.render_widget(get_gauge("AVG", borders_style,  (avg_total / 100.0) as f64), chunks[n]);
                        break;
                    }
                }
            }

            if opts.display_temperature == true {
                // There should be a neater way to handle this..
                // Also, it would be nice if we could set like normal, high and critical areas to
                // the gauge.. or something..
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![Constraint::Percentage(100); 1].as_ref())
                    .split(chunks[row_count - 1]);

                app.get_cpu_temp(); 

                f.render_widget(get_gauge("TEMP", borders_style, 0.5), chunks[0]);
            }

        })?;
    }
    Ok(())
}
