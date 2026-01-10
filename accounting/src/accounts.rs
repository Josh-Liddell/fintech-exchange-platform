use crate::errors::AccountingError;
use crate::tx::Tx;
use std::collections::HashMap;

/// A type for managing accounts and their current currency balance
#[derive(Debug)]
pub struct Accounts {
    accounts: HashMap<String, u64>,
}

impl Accounts {
    /// Returns an empty instance of the [`Accounts`] type
    pub fn new() -> Self {
        Accounts {
            accounts: Default::default(),
        }
    }

    /// Either creates a new account and deposits the `amount` provided into the `signer` or adds the amount to the existing account.
    /// # Errors
    /// Attempted overflow
    pub fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_add(amount)
                .and_then(|r| {
                    *account = r;
                    Some(r)
                })
                .ok_or(AccountingError::AccountOverFunded(
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
    pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountingError> {
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
                .ok_or(AccountingError::AccountUnderFunded(
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
            Err(AccountingError::AccountNotFound(signer.to_string()))
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
    ) -> Result<(Tx, Tx), AccountingError> {
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
                Err(AccountingError::AccountNotFound(sender.to_string()))
            } else {
                Err(AccountingError::AccountNotFound(recipient.to_string()))
            }
        }
        // my first attempt was just returning this:
        // Ok((
        //     self.withdraw(sender, amount)?,
        //     self.deposit(recipient, amount)?,

        //     // but what if you withdraw and then the deposit fails?? they need their money back.
        // ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_errors_handled() {
        // missing, underfunded, or an overflow
        // assert eq with an accounting error
        let mut ledger = Accounts::new();
        let _tx1 = ledger.deposit("joshua", 10).unwrap();

        // underfunded
        assert!(ledger.withdraw("joshua", 20).is_err());

        // overfunded
        assert!(ledger.deposit("joshua", u64::MAX).is_err());

        //missing account
        assert!(ledger.withdraw("josh", 20).is_err());
        assert!(ledger.send("jeremy", "joshua", 20).is_err());
    }

    // keep adding all the types of variants!
    #[test]
    fn correct_transaction_variants() {
        let mut ledger = Accounts::new();
        assert_eq!(
            ledger.deposit("joshua", 10).unwrap(),
            Tx::Deposit {
                account: "joshua".to_string(),
                amount: 10
            }
        );
    }
}
