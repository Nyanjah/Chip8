use std::io::{Stdout, self};
use tui::{widgets::StatefulWidget, Terminal, backend::CrosstermBackend};
use crossterm::{self, terminal::{enable_raw_mode, EnterAlternateScreen}, execute};
use tui::{widgets::{Block, Borders}, layout::{Layout, Direction, Constraint}, style::{Color, Style}};
use tui_logger::{TuiLoggerWidget, TuiLoggerLevelOutput};

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
            /* divide screen for the logger and display */
            let rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(frame.size());
        
        /* draw the display */
        frame.render_stateful_widget(ChipRenderWidget, rects[0],display);
        
        let tui_w = TuiLoggerWidget::default()
            .block(
                Block::default()
                    .title(" Log ")
                    .border_style(Style::default().fg(Color::White).bg(Color::Black))
                    .borders(Borders::ALL),
            )
            .output_separator('|')
            .output_timestamp(None)
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_target(false)
            .output_file(false)
            .output_line(false)
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Cyan))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::White))
            .style_info(Style::default().fg(Color::Green));
        
        /* draw the logger */
        frame.render_widget(tui_w, rects[1]);
            
        }).expect("Failed to render display");
    }

}
