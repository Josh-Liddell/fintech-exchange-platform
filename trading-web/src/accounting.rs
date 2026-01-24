use std::collections::HashMap;
use trading_common::{errors::ApplicationError, tx::Tx};

/// A type for managing accounts and their current currency balance
#[derive(Debug)]
pub struct Accounts {
    pub accounts: HashMap<String, u64>,
}

impl Accounts {
    /// Returns an empty instance of the [`Accounts`] type
    pub fn new() -> Self {
        Accounts {
            accounts: HashMap::new(),
        }
    }

    /// Retrieves the balance of an account
    pub fn balance_of(&self, signer: &str) -> Result<&u64, ApplicationError> {
        self.accounts
            .get(signer)
            .ok_or(ApplicationError::AccountNotFound(signer.to_string()))
    }

    /// Either creates a new account and deposits the `amount` provided into the `signer` or adds the amount to the existing account.
    /// # Errors
    /// Attempted overflow
    pub fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, ApplicationError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_add(amount)
                .and_then(|r| {
                    *account = r;
                    Some(r)
                })
                .ok_or(ApplicationError::AccountOverFunded(
                    signer.to_string(),
                    amount,
                ))
                // Using map() here is an easy way to only manipulate the non-error result
                .map(|_| Tx::Deposit {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            self.accounts.insert(signer.to_string(), amount);
            Ok(Tx::Deposit {
                account: signer.to_string(),
                amount,
            })
        }
    }

    /// Withdraws the `amount` from the `signer` account.
    /// # Errors
    /// Attempted overflow
    pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, ApplicationError> {
        // verify exists, return error otherwise
        // self.acccounts is where they are
        if let Some(account) = self.accounts.get_mut(signer) {
            // the data type u64 cannot be negative and unchecked subtraction can crash your program.
            (*account)
                .checked_sub(amount)
                .and_then(|r| {
                    *account = r;
                    Some(r)
                })
                .ok_or(ApplicationError::AccountUnderFunded(
                    signer.to_string(),
                    amount,
                ))
                // Using map() here is an easy way to only manipulate the non-error result
                // 'looping' once (or not at all if None) in order to change the Some(r) into a Tx::Withdraw, that gets returned.
                .map(|_| Tx::Withdraw {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            Err(ApplicationError::AccountNotFound(signer.to_string()))
        }
    }

    /// Withdraws the amount from the sender account and deposits it in the recipient account.
    ///
    /// # Errors
    /// The account doesn't exist
    pub fn send(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<(Tx, Tx), ApplicationError> {
        if self.accounts.contains_key(sender)
            && self.accounts.contains_key(recipient)
            && self
                .accounts
                .get(sender)
                .map(|amt| *amt >= amount)
                .unwrap_or(false)
        {
            let tx_withdraw = self.withdraw(sender, amount)?;
            self.deposit(recipient, amount)
                .map_err(|e| {
                    self.deposit(sender, amount).unwrap();
                    e
                })
                .map(|tx_deposit| (tx_withdraw, tx_deposit))
        } else {
            if !self.accounts.contains_key(sender) {
                Err(ApplicationError::AccountNotFound(sender.to_string()))
            } else {
                Err(ApplicationError::AccountNotFound(recipient.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accounts_withdraw_underfunded() {
        let mut accounts = Accounts::new();
        accounts.deposit("a-key", 0).unwrap();
        let actual = accounts.withdraw("a-key", 100);
        assert_eq!(
            actual,
            Err(ApplicationError::AccountUnderFunded(
                "a-key".to_string(),
                100
            ))
        );
    }

    #[test]
    fn test_accounts_deposit_overfunded() {
        let mut accounts = Accounts::new();
        accounts
            .deposit("a-key", 1)
            .expect("Initial deposit failed");
        let actual = accounts.deposit("a-key", u64::MAX);
        assert_eq!(
            actual,
            Err(ApplicationError::AccountOverFunded(
                "a-key".to_string(),
                u64::MAX
            ))
        );
    }

    #[test]
    fn test_accounts_deposit_works() {
        let mut accounts = Accounts::new();
        let amt = 100;
        let actual = accounts.deposit("a-key", amt);
        assert_eq!(
            actual,
            Ok(Tx::Deposit {
                account: "a-key".to_string(),
                amount: amt
            })
        );
    }

    #[test]
    fn test_accounts_withdraw_works() {
        let mut accounts = Accounts::new();
        let amt = 100;
        accounts.deposit("a-key", amt).expect("Couldn't deposit");
        let actual = accounts.withdraw("a-key", amt);
        assert_eq!(
            actual,
            Ok(Tx::Withdraw {
                account: "a-key".to_string(),
                amount: amt
            })
        );
    }

    #[test]
    fn test_accounts_send_works() {
        let mut accounts = Accounts::new();
        let amt = 100;
        accounts.deposit("a-key", amt).expect("Couldn't deposit");

        // creating the receiver is also required
        accounts.deposit("b-key", 0).expect("Couldn't deposit");

        let (tx1, tx2) = accounts.send("a-key", "b-key", amt).expect("Send failed");
        assert_eq!(
            tx1,
            Tx::Withdraw {
                account: "a-key".to_string(),
                amount: amt
            }
        );
        assert_eq!(
            tx2,
            Tx::Deposit {
                account: "b-key".to_string(),
                amount: amt
            }
        );

        let actual = accounts.withdraw("b-key", amt);
        assert_eq!(
            actual,
            Ok(Tx::Withdraw {
                account: "b-key".to_string(),
                amount: amt
            })
        );
    }

    #[test]
    fn test_accounts_send_underfunded_fails_and_rolls_back() {
        let mut accounts = Accounts::new();
        let amt = 100;
        accounts.deposit("a-key", amt).expect("Couldn't deposit");

        // creating the receiver is also required
        accounts.deposit("b-key", 0).expect("Couldn't deposit");

        let actual = accounts.send("a-key", "b-key", amt + 1);
        assert!(actual.is_err());
        let expected: HashMap<String, u64> =
            vec![("a-key".to_string(), amt), ("b-key".to_string(), 0)]
                .into_iter()
                .collect();
        assert_eq!(accounts.accounts, expected);
    }

    #[test]
    fn test_accounts_send_overfunded_fails_and_rolls_back() {
        let mut accounts = Accounts::new();
        let amt = 100;
        accounts.deposit("a-key", amt).expect("Couldn't deposit");

        // creating the receiver is also required
        accounts
            .deposit("b-key", u64::MAX)
            .expect("Couldn't deposit");

        let actual = accounts.send("a-key", "b-key", 1);
        assert!(actual.is_err());
        let expected: HashMap<String, u64> =
            vec![("a-key".to_string(), amt), ("b-key".to_string(), u64::MAX)]
                .into_iter()
                .collect();
        assert_eq!(accounts.accounts, expected);
    }
}
