use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;
use std::iter;

const INPUT_FILE_PATH: &str = "data/day14_input.txt";
const WIDTH: usize = 101;
const HEIGHT: usize = 103;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let contents = std::fs::read_to_string(INPUT_FILE_PATH).expect("Input file not readable");
    let robots = parse(&contents);
    let app_result = App::new(robots).run(&mut terminal);
    ratatui::restore();
    app_result
}

struct Robot {
    p_x: i128,
    p_y: i128,
    v_x: i128,
    v_y: i128,
}

impl Robot {
    fn get_position(
        &self,
        step_count: usize,
        area_width: usize,
        area_height: usize,
    ) -> (usize, usize) {
        let n = step_count as i128;
        let w = area_width as i128;
        let h = area_height as i128;
        let mut final_x = (self.p_x + self.v_x * n) % w;
        let mut final_y = (self.p_y + self.v_y * n) % h;
        if final_x < 0 {
            final_x += w;
        }
        if final_y < 0 {
            final_y += h;
        }
        (final_x as usize, final_y as usize)
    }
}

pub struct App {
    robots: Vec<Robot>,
    steps: usize,
    exit: bool,
}

impl App {
    fn new(robots: Vec<Robot>) -> Self {
        Self {
            robots,
            steps: 0,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_steps(),
            KeyCode::Right => self.increment_steps(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_steps(&mut self) {
        self.steps += 1;
    }

    fn decrement_steps(&mut self) {
        if self.steps > 0 {
            self.steps -= 1;
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Advent of Code 2024: Day 14 Part 2 ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let grid = get_grid(&self.robots, self.steps);
        let mut grid_rows: Vec<Line> = grid
            .iter()
            .map(|s| Line::from(s.as_str()).green())
            .collect();
        grid_rows.insert(
            0,
            Line::from(vec!["Steps: ".into(), self.steps.to_string().yellow()]),
        );
        let text = Text::from(grid_rows);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn parse(contents: &str) -> Vec<Robot> {
    contents.lines().map(parse_robot).collect()
}

fn parse_robot(line: &str) -> Robot {
    let (left, right) = line.split_once(' ').unwrap();
    let (p_x, p_y) = parse_pair(left);
    let (v_x, v_y) = parse_pair(right);
    Robot { p_x, p_y, v_x, v_y }
}

fn parse_pair(coordinates: &str) -> (i128, i128) {
    let (left, right) = coordinates
        .split_once('=')
        .unwrap()
        .1
        .split_once(',')
        .unwrap();
    (
        left.parse::<i128>().unwrap(),
        right.parse::<i128>().unwrap(),
    )
}

fn get_grid(robots: &[Robot], steps: usize) -> Vec<String> {
    let row_template: Vec<u8> = iter::repeat(b'.').take(WIDTH).collect();
    let mut grid_u8 = vec![row_template; HEIGHT];
    robots.iter().for_each(|robot| {
        let pos = robot.get_position(steps, WIDTH, HEIGHT);
        grid_u8[pos.1][pos.0] = b'*';
    });
    let grid: Vec<String> = grid_u8
        .iter()
        .map(|row| String::from_utf8(row.clone()).unwrap())
        .collect();
    grid
}
