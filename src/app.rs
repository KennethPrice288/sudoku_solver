
use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
};

use crossterm::event::KeyEventKind;
use ratatui::{
    backend::CrosstermBackend, crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    }, layout::{Constraint, Layout}, style::{Color, Style, Stylize}, symbols::Marker, terminal::{Frame, Terminal}, text, widgets::{
        canvas::{Canvas, Line, Rectangle}, Block, Paragraph, Widget
    },
};

use crate::sudoku;

const NUMBER_SPACE : f64 = 900.0;
const INFO_TEXT: &str =
    "(q) quit | (↑) move up | (↓) move down | (→) move right | (←) move left | (1-9) input value | (enter) solve";

pub(crate) struct App<'a> {
    x: f64,
    y: f64,
    tick_count: u64,
    marker: Marker,
    board: &'a mut [[i32; 9]; 9],
}

impl<'a> App <'a> {
    pub fn new(board: &'a mut [[i32; 9]; 9]) -> Self {
        Self {
            x: 1.5,
            y: 1.5,
            tick_count: 0,
            marker: Marker::Block,
            board,
        }
    }

    fn set_x(&mut self, x: f64) {
        self.x = x.clamp(-4.5, 3.5);
    }

    fn set_y(&mut self, y: f64) {
        self.y = y.clamp(-3.5, 4.5);
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = init_terminal()?;
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(16);
        loop {
            let _ = terminal.draw(|frame| self.ui(frame));
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Down | KeyCode::Char('j') => self.set_y(self.y + 1.0),
                            KeyCode::Up | KeyCode::Char('k') => self.set_y(self.y - 1.0),
                            KeyCode::Right | KeyCode::Char('l') => self.set_x(self.x + 1.0),
                            KeyCode::Left | KeyCode::Char('h') => self.set_x(self.x - 1.0),
                            KeyCode::Char('1') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 1),
                            KeyCode::Char('2') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 2),
                            KeyCode::Char('3') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 3),
                            KeyCode::Char('4') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 4),
                            KeyCode::Char('5') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 5),
                            KeyCode::Char('6') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 6),
                            KeyCode::Char('7') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 7),
                            KeyCode::Char('8') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 8),
                            KeyCode::Char('9') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 9),
                            KeyCode::Char('0') => sudoku::user_input(self.board, (self.x + 5.5) as i32, (self.y + 4.5) as i32, 0),
                            KeyCode::Enter => sudoku::get_solution(self.board),
                            _ => {}
                        }
                    }
                }
            }
            
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        restore_terminal()
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
    }

    fn ui(&self, frame: &mut Frame) {
        let rects = Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).split(frame.size());
        frame.render_widget(self.sudoku_canvas(), rects[0]);
        let info_footer = Paragraph::new(text::Line::from(INFO_TEXT))
            .style(Style::new().fg(Color::Magenta))
            .centered()
            .block(
                Block::bordered()
                .border_type(ratatui::widgets::BorderType::Double)
                .border_style(Style::new().fg(Color::White))
            );
        frame.render_widget(info_footer, rects[1]);
    }

    fn sudoku_canvas(&self) -> impl Widget + '_ {


        Canvas::default()
        .block(Block::bordered().title("Sudoku"))
        .marker(self.marker)
        .paint(|ctx| {
            let style = Style::new().white();

            for x in 0..=9 {
                if x%3 != 0 {
                    ctx.draw(&Line {
                        x1: -(NUMBER_SPACE * 9.0 / 2.0) + (NUMBER_SPACE * x as f64),
                        x2: -(NUMBER_SPACE * 9.0 / 2.0) + (NUMBER_SPACE * x as f64),
                        y1: -(NUMBER_SPACE * 9.0 / 2.0),
                        y2:  (NUMBER_SPACE * 9.0 / 2.0),
                        color: Color::DarkGray
                        });
                }
            }

            for y in 0..=9 {
                if y%3 != 0 {
                    ctx.draw(&Line {
                        x1 : (NUMBER_SPACE * 9.0 / 2.0),
                        x2 : -(NUMBER_SPACE * 9.0 / 2.0),
                        y1 : -(NUMBER_SPACE * 9.0 / 2.0) + (NUMBER_SPACE * y as f64),
                        y2 : -(NUMBER_SPACE * 9.0 / 2.0) + (NUMBER_SPACE * y as f64),
                        color: Color::DarkGray
                        })
                }
            }

            for x in 0..=9 {
                if x%3 == 0 {
                    ctx.draw(&Line {
                        x1: -(NUMBER_SPACE * 9.0 / 2.0) + (NUMBER_SPACE * x as f64),
                        x2: -(NUMBER_SPACE * 9.0 / 2.0) + (NUMBER_SPACE * x as f64),
                        y1: -(NUMBER_SPACE * 9.0 / 2.0),
                        y2:  (NUMBER_SPACE * 9.0 / 2.0),
                        color: Color::White
                        });
                }
            }

            for y in 0..=9 {
                if y%3 == 0 {
                    ctx.draw(&Line {
                        x1 : (NUMBER_SPACE * 9.0 / 2.0),
                        x2 : -(NUMBER_SPACE * 9.0 / 2.0),
                        y1 : -(NUMBER_SPACE * 9.0 / 2.0) + (NUMBER_SPACE * y as f64),
                        y2 : -(NUMBER_SPACE * 9.0 / 2.0) + (NUMBER_SPACE * y as f64),
                        color: Color::White
                        })
                }
            }
            
            ctx.draw(&Rectangle {
                x : self.x * NUMBER_SPACE,
                y : -self.y * NUMBER_SPACE,
                width : NUMBER_SPACE,
                height : NUMBER_SPACE,
                color: Color::Blue
            });

            for x in 0..9 {
                for y in 0..9 {
                    let num = text::Line::styled(format!("{}", self.board[x][y]), style);
                    ctx.print((x as f64 - 4.0) * NUMBER_SPACE, -(y as f64 - 4.0) * NUMBER_SPACE, num);
                }
            }

        })
        
        .x_bounds([-(NUMBER_SPACE * 9.0 / 2.0), (NUMBER_SPACE * 9.0 / 2.0)])
        .y_bounds([-(NUMBER_SPACE * 9.0 / 2.0), (NUMBER_SPACE * 9.0 / 2.0)])
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
