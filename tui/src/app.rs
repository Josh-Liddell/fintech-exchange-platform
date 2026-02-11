use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use trading_common::{PartialOrder, requests::OrderBookResponse};

// the counter and incrementing and decrementing was from the example

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,

    pub bids: Vec<PartialOrder>,
    pub asks: Vec<PartialOrder>,
    pub tickct: u64,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            bids: Vec::new(),
            asks: Vec::new(),
            tickct: 0,
            events: EventHandler::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?; //we are making the APP itself a widget and redering it here
            // terminal.draw(ui::render)?;
            match self.events.next().await? {
                Event::Tick => self.tick().await,
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    // AppEvent::Increment => self.increment_counter(),
                    // AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // KeyCode::Right => self.events.send(AppEvent::Increment),
            // KeyCode::Left => self.events.send(AppEvent::Decrement),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub async fn tick(&mut self) {
        // tick stays at 30 times a second
        // the poll here is happening every 30 ticks or once every second
        // this could be bad? maybe do it a different way to decouple from using the tick loop
        // seems like a lot of unecesary calculations I better use tokio iterval or something like that.
        self.tickct += 1;
        if self.tickct % 30 == 0 {
            let resp: OrderBookResponse = reqwest::get("http://localhost:8080/orderbook")
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            let OrderBookResponse { mut asks, mut bids } = resp;
            asks.sort_by(|a, b| a.price.cmp(&b.price));
            bids.sort_by(|a, b| b.price.cmp(&a.price));

            self.asks = asks;
            self.bids = bids;
        }
    } // poll server and set self.bids and asks to be to that data?

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    // pub fn increment_counter(&mut self) {
    //     // self.counter = self.counter.saturating_add(1);
    //     let order = PartialOrder {
    //         price: 12,
    //         amount: 10,
    //         remaining: 10,
    //         side: Side::Buy,
    //         signer: "Joshua".to_string(),
    //         ordinal: 1,
    //     };
    //     self.bids.push(order.clone());
    //     self.asks.push(order.clone());
    // }

    // pub fn decrement_counter(&mut self) {
    //     self.bids.pop();
    //     self.asks.pop();
    // }
}
