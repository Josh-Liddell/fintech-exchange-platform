use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Row, Table, Widget},
};
use trading_common::{PartialOrder, Side};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let main_layout =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]);
        let left_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let [top, main] = area.layout(&layout);
        let [left, _right] = main.layout(&main_layout);
        let [left_top, left_bottom] = left.layout(&left_layout);

        let title = Line::from_iter([
            Span::from("Limit Order Book").bold(),
            Span::from(" (Press 'q' to quit)"),
        ]);

        // create the other things I guess (when I flipped these it worked??)
        let ask_table = build_table(&self.asks, Side::Sell);
        let bid_table = build_table(&self.bids, Side::Buy);

        // render the things
        title.centered().render(top, buf);
        ask_table.render(left_top, buf);
        bid_table.render(left_bottom, buf);
        // orderform.render(right, buf)
    }
}

fn build_table(orders: &[PartialOrder], side: Side) -> Table {
    let (table_style, title) = match side {
        Side::Buy => (Style::new().fg(Color::Green), "Buy Orders"),
        Side::Sell => (Style::new().fg(Color::Red), "Sell Orders"),
    };

    let block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    // table data
    let header = Row::new(["Price", "Remaining", "Account", "Order #"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let rows: Vec<Row> = orders
        .iter()
        // .map(|r| Row::new(r).style(table_style))
        .map(|r| {
            Row::new([
                r.price.to_string(),
                r.remaining.to_string(),
                r.signer.clone(),
                r.ordinal.to_string(),
            ])
            .style(table_style)
        })
        .collect();

    let widths = [Constraint::Percentage(25); 4];

    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(1)
        .block(block)
        .style(Color::LightBlue);

    table
}
