use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountUpdateRequest {
    pub signer: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AccountBalanceRequest {
    pub signer: String,
}

#[derive(Serialize, Deserialize)]
pub struct SendRequest {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}
