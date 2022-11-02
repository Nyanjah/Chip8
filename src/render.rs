use std::io::{Stdout, self};

use tui::{widgets::StatefulWidget, Terminal, backend::CrosstermBackend};
use crossterm::{self, terminal::{enable_raw_mode, EnterAlternateScreen}, execute};

pub struct ChipRender{
    terminal:Terminal<CrosstermBackend<Stdout>>
}

struct ChipRenderWidget;

impl StatefulWidget for ChipRenderWidget {
    type State = [[bool; 32]; 64];
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer,state: &mut Self::State) {
        for x in 0..64{
            for y in 0..32{
                // Making sure x and y fall withing the terminal height and width
                if x < area.width && y < area.height  {
                buf.get_mut(x,y).set_bg(if state[x as usize][y as usize] {tui::style::Color::Green} else {tui::style::Color::Black});
                }
            }
        }
    }
}

impl ChipRender{
    pub fn setup()->Result<ChipRender,io::Error>{
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout,EnterAlternateScreen)?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout))?;
        Ok(ChipRender{terminal})
    }

    pub fn render(& mut self,display:& mut[[bool; 32]; 64]) -> () {
    
        self.terminal.draw(|frame|{
            frame.render_stateful_widget(ChipRenderWidget, frame.size(),display)
        }).expect("Failed to render display");
    }

}

