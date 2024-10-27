use std::io;
use rand::Rng;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Stylize,Color, Style},
    symbols::border,
    text::{Line, Text},
    widgets::{
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};



fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    score: u8,
    exit: bool,
    snake: Snake,
    apple: Apple
}

#[derive(Debug)]
pub struct Snake {
    position:  (u16, u16),
    unit: String,
}

impl Default for Snake {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position: (rng.gen_range(0..168), rng.gen_range(0..15)), // x and y are random
            unit: "🟩".to_string(),
        }
    }
}


impl Snake{
    fn move_right(&mut self) {
        self.position.0 += 1;
    }

    fn move_left(&mut self) {
        if self.position.0 == 0 {
            return;
        }
        self.position.0  -= 1;

    }
    fn move_up(&mut self) {
        if self.position.1 == 0 {
            return;
        }
        self.position.1  -= 1;
    }
    fn move_down(&mut self) {
        
        self.position.1  += 1;
    }

    
}
#[derive(Debug)]
pub struct Apple {
    position: (u16, u16), // (x, y) x is left and right, y is up and down. generated randomly
    unit: String,
}

impl Default for Apple {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position: (rng.gen_range(0..168), rng.gen_range(0..15)), // x and y are random
            unit: "🍎".to_string(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            score: 0,
            exit: false,
            snake: Snake::default(),
            apple: Apple::default(),
        }
    }
}

impl App {
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
            KeyCode::Left => self.snake.move_left(),
            KeyCode::Right => self.snake.move_right(),
            KeyCode::Down => self.snake.move_down(),
            KeyCode::Up => self.snake.move_up(),
            _ => {}
        }
        self.eat_apple();
    }

    fn exit(&mut self) {
        self.exit = true;
    }

   fn eat_apple(&mut self) {
       if self.snake.position == self.apple.position {
           self.score += 1;
           self.apple = Apple::default();
           self.snake.unit += "🟩";
       }
   }

  
}


impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let instructions = Line::from(vec![
            " Move Left ".into(),
            "⬅️".blue().bold(),
            " Move Right ".into(),
            "➡️".blue().bold(),
            " Move Down ".into(),
            "⬇️".blue().bold(),
            " Move Up ".into(),
            "⬆️".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered().title_top(" Score ".bold()).title_alignment(Alignment::Center)
            .title_bottom(instructions).title_alignment(Alignment::Center)
          
            .border_set(border::THICK);

        let score_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.score.to_string().yellow(),
        ])]);

        Paragraph::new(score_text)
            .centered()
            .block(block)
            .render(area, buf);

        // draw the player
        buf.set_string(self.snake.position.0 as u16, self.snake.position.1 as u16, &self.snake.unit, Style::new());

        // draw the apple
        buf.set_string(self.apple.position.0 as u16, self.apple.position.1 as u16, &self.apple.unit, Style::new());

      


    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eat_apple() {
        let mut app = App::default();
        app.snake.position = app.apple.position;
        app.eat_apple();
        assert_eq!(app.score, 1);
    }
}