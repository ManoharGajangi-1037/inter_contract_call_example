use cw_storage_plus::{Item, Map};
use cosmwasm_std::Addr;

pub const MESSAGES: Item<Vec<String>> = Item::new("messages");

// Address of contract A (manually set after deployment or on init)
pub const AUTHORIZED_CONTRACT: Item<Addr> = Item::new("authorized");
