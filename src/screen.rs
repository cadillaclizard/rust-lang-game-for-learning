use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new(titles: Vec<String>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

// impl<B> Drop for Screen<B> {
//     fn drop(&mut self) {
//         // restore terminal
//         disable_raw_mode()?;
//         execute!(
//             terminal.backend_mut(),
//             LeaveAlternateScreen,
//             DisableMouseCapture
//         )?;
//         self.terminal.show_cursor()?;

//         // if let Err(err) = res {
//         //     println!("{err:?}");
//         // }
//     }
// }

// pub fn start_terminal() -> io::Result<Screen> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
//     let backend: Backend = CrosstermBackend::new(stdout);
//     // let mut terminal = Terminal::new(backend).unwrap();

//     Ok(Screen {
//         backend,
//         terminal: Terminal::new(backend).as_mut(),
//     })
// }

// pub fn init() -> Result<()> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend);

//     Ok(())
// }
// }

pub struct Screen {
    pub should_quit: bool,
    pub tabs: TabsState,
    pub progress: f64,
}

impl Screen {
    pub fn new() {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend);

        terminal;
    }

    // pub fn close(&mut self, terminal: Terminal<dyn Backend>) {
    //     // // create app and run it
    //     // let app = App::new("Crossterm Demo", enhanced_graphics);
    //     // let res = run_app(&mut terminal, app, tick_rate);

    //     // restore terminal
    //     disable_raw_mode()?;
    //     execute!(
    //         terminal.backend_mut(),
    //         LeaveAlternateScreen,
    //         DisableMouseCapture
    //     )?;
    //     terminal.show_cursor()?;

    //     // if let Err(err) = res {
    //     //     println!("{err:?}");
    //     // }
    // }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }
    }
}

// pub fn run(tick_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Screen::new(backend)?;

//     // create app and run it
//     let app = App::new("Crossterm Demo", enhanced_graphics);
//     let res = run_app(&mut terminal, app, tick_rate);

//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     if let Err(err) = res {
//         println!("{err:?}");
//     }

//     Ok(())
// }

// fn run_app<B: Backend>(
//     terminal: &mut Screen<B>,
//     mut app: App,
//     tick_rate: Duration,
// ) -> io::Result<()> {
//     let mut last_tick = Instant::now();
//     loop {
//         terminal.draw(|f| ui::draw(f, &mut app))?;

//         let timeout = tick_rate
//             .checked_sub(last_tick.elapsed())
//             .unwrap_or_else(|| Duration::from_secs(0));
//         if crossterm::event::poll(timeout)? {
//             if let Event::Key(key) = event::read()? {
//                 if key.kind == KeyEventKind::Press {
//                     match key.code {
//                         KeyCode::Char(c) => app.on_key(c),
//                         KeyCode::Left => app.on_left(),
//                         KeyCode::Up => app.on_up(),
//                         KeyCode::Right => app.on_right(),
//                         KeyCode::Down => app.on_down(),
//                         _ => {}
//                     }
//                 }
//             }
//         }
//         if last_tick.elapsed() >= tick_rate {
//             app.on_tick();
//             last_tick = Instant::now();
//         }
//         if app.should_quit {
//             return Ok(());
//         }
//     }
// }

// pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
//     let tabs = TabsState::new(vec!["Tab0", "Tab1", "Tab2"]);

//     let chunks = Layout::default()
//         .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
//         .split(f.size());
//     let titles = app
//         .tabs
//         .titles
//         .iter()
//         .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
//         .collect();
//     let tabs = Tabs::new(titles)
//         .block(Block::default().borders(Borders::ALL).title(app.title))
//         .highlight_style(Style::default().fg(Color::Yellow))
//         .select(app.tabs.index);
//     f.render_widget(tabs, chunks[0]);
//     // match app.tabs.index {
//     //     0 => draw_first_tab(f, app, chunks[1]),
//     //     1 => draw_second_tab(f, app, chunks[1]),
//     //     2 => draw_third_tab(f, app, chunks[1]),
//     //     _ => {}
//     // };
// }
