use super::common::*;

pub const PARALLEL_LIMIT: usize = 50;
pub type HolderResults = Vec<Result<Holder>>;

#[derive(Debug, Serialize, Clone)]
pub struct Holder {
    pub owner_wallet: String,
    pub associated_token_address: String,
    pub mint_account: String,
    pub metadata_account: String,
}

#[derive(Debug, Serialize)]
pub struct TarsProgramAccounts {
    pub config_accounts: Vec<ConfigAccount>,
    pub tars_accounts: Vec<TarsAccount>,
}

#[derive(Debug, Serialize)]
pub struct ConfigAccount {
    pub address: String,
    pub data_len: usize,
}

#[derive(Debug, Serialize)]
pub struct TarsAccount {
    pub address: String,
    pub data_len: usize,
}

pub struct SnapshotMintsArgs {
    pub creator: Option<String>,
    pub position: usize,
    pub update_authority: Option<String>,
    pub v2: bool,
    pub output: String,
}
