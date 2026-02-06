use std::collections::{BTreeMap, BinaryHeap};
use trading_common::{Order, PartialOrder, Receipt, Side, errors::ApplicationError};

#[derive(Default, Debug)]
pub struct MatchingEngine {
    /// The last sequence number
    pub ordinal: u64,

    /// The "Bid" or "Buy" side of the order book. Ordered by ordinal number.
    // orders in the binary heap ordered by ordinal number
    // it was impled to be revered so we can pop off the end, and if you push it orders it correctly
    pub bids: BTreeMap<u64, BinaryHeap<PartialOrder>>,
    /// The "Ask" or "Sell" side of the order book. Ordered by ordinal number.
    pub asks: BTreeMap<u64, BinaryHeap<PartialOrder>>,

    /// Previous matches for record keeping
    pub history: Vec<Receipt>,
}

impl MatchingEngine {
    /// Creates a new [`MatchingEngine`] with an ordinal of 0 and empty books
    pub fn new() -> Self {
        MatchingEngine {
            ordinal: 0,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            history: Vec::new(),
        }
    }

    /// Processes an [`Order`] and returns a [`Receipt`]
    /// This includes matching the order to whatever is in the current books and adding the remainder (if any) to the book for future matching.
    pub fn process(&mut self, order: Order) -> Result<Receipt, ApplicationError> {
        // Increment the ordinal number for this order
        self.ordinal += 1;
        let ordinal = self.ordinal;

        let original_amount = order.amount;
        let mut partial = order.into_partial_order(ordinal, original_amount);

        // Orders are matched to the opposite side
        let receipt = match &partial.side {
            Side::Buy => {
                // This is the code for processing a buy order

                // first we are looking at the existing ask orders that are possible to match with, (applicable price range)
                // in this case asks that are equal to or less than our bid
                // we get an iterator over that part of the ask book
                let orderbook_entry = self.asks.range_mut(u64::MIN..=partial.price);

                // we pass the range iterator (prices and associated orders) and this new order to be matched with it
                let receipt = MatchingEngine::match_order(&partial, orderbook_entry, ordinal)?;

                // we sum up the qty that was matched with this order
                let matched_amount: u64 = receipt
                    .matches
                    .iter()
                    .map(|matched_ordr| matched_ordr.amount)
                    .sum();

                // if quantity is less than original then its not fully matched
                // and we need to add it to the bids with its remaining amount
                if matched_amount < original_amount {
                    partial.amount = original_amount - matched_amount;
                    partial.remaining = original_amount - matched_amount;
                    let price = partial.price;
                    let bids = self.bids.entry(price).or_insert(vec![].into());
                    bids.push(partial);
                }
                receipt
            }
            Side::Sell => {
                let orderbook_entry = self.bids.range_mut(partial.price..=u64::MAX).rev();

                let receipt = MatchingEngine::match_order(&partial, orderbook_entry, ordinal)?;
                let matched_amount: u64 = receipt.matches.iter().map(|m| m.amount).sum();

                if matched_amount < original_amount {
                    partial.amount = original_amount - matched_amount;
                    partial.remaining = original_amount - matched_amount;
                    let price = partial.price;
                    let asks = self.asks.entry(price).or_insert(vec![].into());
                    asks.push(partial);
                }
                receipt
            }
        };

        // Cleanup: Remove price entries without orders from the orderbook
        self.asks.retain(|_, orders| !orders.is_empty());
        self.bids.retain(|_, orders| !orders.is_empty());

        // Keep a log of matches
        self.history.push(receipt.clone());
        Ok(receipt)
    }

    /// Matches an order to the provided order book side.
    /// # Parameters
    /// - `order`: the order to match to the book
    /// - `orderbook_entry`: a pre-filtered iterator for order book_entry in the correct price range
    /// - `ordinal` the next ordinal number to use if a position is opened
    fn match_order<'a, T>(
        order: &PartialOrder,
        mut orderbook_iterator: T,
        ordinal: u64,
    ) -> Result<Receipt, ApplicationError>
    where
        T: Iterator<Item = (&'a u64, &'a mut BinaryHeap<PartialOrder>)>,
    {
        // qty that needs to be matched
        let mut remaining_amount = order.amount;
        let mut matches = vec![];

        // while there is still quantity to be matched
        'outer: while remaining_amount > 0 {
            // The iterator contains all orderbook_entry of a price point
            match orderbook_iterator.next() {
                Some((price, orderbook_entries)) => {
                    let mut self_matches = vec![];

                    // nested loop getting the next partial order with lowest ordinal
                    'entry_loop: while let Some(mut entry) = orderbook_entries.pop() {
                        if order.signer == entry.signer {
                            // self match got taken off the binary heap so it needs to get added back on later
                            self_matches.push(entry);
                            continue 'entry_loop;
                        }

                        // we know its a match now so we
                        // try and decrese the existing order by how much our incoming order needs
                        match entry.remaining.checked_sub(remaining_amount) {
                            // say the existing order had 20 qty and we have 5 left to match, subtracting 5 wont cause error
                            // that means this existing order fulfilled what we needed and we are done
                            // our remaining_amount 'ran out'
                            Some(_) => {
                                // we edit the existing order, subtracting its remaining amount, and
                                // create a new order that represents what we matched with
                                matches.push(PartialOrder::take_from(
                                    &mut entry,
                                    remaining_amount,
                                    *price,
                                ));

                                // after subtracting from it, if the existing order still has qty is goes back on the heap
                                if entry.remaining > 0 {
                                    orderbook_entries.push(entry);
                                }

                                remaining_amount = 0;
                                // order is fully matched so don't check more orders
                                break 'entry_loop;
                            }

                            // if checked sub fails that means we are not done, our remaining_amount exceeded what this order could give
                            // this means that this existing order fully matched, so we push it to matches
                            // (incoming order not fully matched, still qty remaining)
                            None => {
                                let fill_qty = entry.remaining;
                                remaining_amount -= fill_qty;
                                entry.amount = fill_qty;
                                entry.remaining = 0;

                                // add the PartialOrder to your matches and continue
                                matches.push(entry);
                            }
                        }
                    }

                    // put self matches back on binary heap (while we still have orderbook_entries available)
                    self_matches
                        .into_iter()
                        .for_each(|order| orderbook_entries.push(order));
                }
                // Nothing left to match with
                None => break 'outer,
            }
        }
        Ok(Receipt { ordinal, matches })
    }
}

#[cfg(test)]
mod tests {
    // reduce the warnings for naming tests
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn test_MatchingEngine_process_partially_match_order() {
        // Immplement me
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let bob_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 2,
                side: Side::Buy,
                signer: "BOB".to_string(),
            })
            .unwrap();

        assert_eq!(
            bob_receipt.matches,
            vec![PartialOrder {
                price: 10,
                amount: 1,
                remaining: 0,
                side: Side::Sell,
                signer: "ALICE".to_string(),
                ordinal: 1
            }]
        );
        assert_eq!(bob_receipt.ordinal, 2);

        // Alice's order was fulfilled and should now be gone
        assert!(matching_engine.asks.is_empty());

        // A partial order for bob should still remain
        assert_eq!(matching_engine.bids.len(), 1);
    }

    #[test]
    fn test_MatchingEngine_process_fully_match_order() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 2,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let bob_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 2,
                side: Side::Buy,
                signer: "BOB".to_string(),
            })
            .unwrap();

        assert_eq!(
            bob_receipt.matches,
            vec![PartialOrder {
                price: 10,
                amount: 2,
                remaining: 0,
                side: Side::Sell,
                signer: "ALICE".to_string(),
                ordinal: 1
            }]
        );

        // A fully matched order doesn't remain in the book
        assert!(matching_engine.asks.is_empty());
        assert!(matching_engine.bids.is_empty());
    }

    #[test]
    fn test_MatchingEngine_process_fully_match_order_multi_match() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let charlie_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "CHARLIE".to_string(),
            })
            .unwrap();
        assert_eq!(charlie_receipt.matches, vec![]);
        assert_eq!(charlie_receipt.ordinal, 2);

        let bob_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 2,
                side: Side::Buy,
                signer: "BOB".to_string(),
            })
            .unwrap();

        assert_eq!(
            bob_receipt.matches,
            vec![
                PartialOrder {
                    price: 10,
                    amount: 1,
                    remaining: 0,
                    side: Side::Sell,
                    signer: "ALICE".to_string(),
                    ordinal: 1
                },
                PartialOrder {
                    price: 10,
                    amount: 1,
                    remaining: 0,
                    side: Side::Sell,
                    signer: "CHARLIE".to_string(),
                    ordinal: 2
                }
            ]
        );
        // A fully matched order doesn't remain in the book
        assert!(matching_engine.asks.is_empty());
        assert!(matching_engine.bids.is_empty());
    }

    #[test]
    fn test_MatchingEngine_process_fully_match_order_no_self_match() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let charlie_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "CHARLIE".to_string(),
            })
            .unwrap();
        assert_eq!(charlie_receipt.matches, vec![]);
        assert_eq!(charlie_receipt.ordinal, 2);

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 2,
                side: Side::Buy,
                signer: "ALICE".to_string(),
            })
            .unwrap();

        assert_eq!(
            alice_receipt.matches,
            vec![PartialOrder {
                price: 10,
                amount: 1,
                remaining: 0,
                side: Side::Sell,
                signer: "CHARLIE".to_string(),
                ordinal: 2
            }]
        );
        // A fully matched order doesn't remain in the book
        assert_eq!(matching_engine.asks.len(), 1);
        assert_eq!(matching_engine.bids.len(), 1);
    }

    #[test]
    fn test_MatchingEngine_process_no_match() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 2,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let bob_receipt = matching_engine
            .process(Order {
                price: 11,
                amount: 2,
                side: Side::Sell,
                signer: "BOB".to_string(),
            })
            .unwrap();

        assert_eq!(bob_receipt.matches, vec![]);
        assert_eq!(matching_engine.asks.len(), 2);
    }

    #[test]
    fn test_MatchingEngine_process_increment_ordinal_matching_engine() {
        let mut matching_engine = MatchingEngine::new();
        assert_eq!(matching_engine.ordinal, 0);
        let receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Buy,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(receipt.ordinal, matching_engine.ordinal);

        let receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Buy,
                signer: "BOB".to_string(),
            })
            .unwrap();
        assert_eq!(receipt.ordinal, matching_engine.ordinal);

        let receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Buy,
                signer: "CHARLIE".to_string(),
            })
            .unwrap();
        assert_eq!(receipt.ordinal, matching_engine.ordinal);
        assert_eq!(matching_engine.ordinal, 3);
    }
}
