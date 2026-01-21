use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(" Limit order book (press q to quit)")
            .title_alignment(Alignment::Left);
        // .border_type(BorderType::);

        let text = format!(
            "\n\n\nLimit order book table will be below\n\
                \n\nExample code:\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            self.counter
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            // .bg(Color::Black)
            .centered();

        paragraph.render(area, buf);

        // let logo = RatatuiLogo::new(RatatuiLogoSize::Tiny);
        // logo.render(area, buf);
    }
}
