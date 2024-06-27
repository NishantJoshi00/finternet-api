pub struct TokenManagerRef {
    pub id: String,
    pub token_manager_name: String,
    pub internal_addr: String, // better name required
}

pub enum AssetInfo {
    Cash { currency: Currency, amount: u64 },
    // Property {}
}

// TODO: Use macros to generate the following enums
pub enum AssetType {
    Cash,
    // Property
}

pub enum Currency {
    USD,
}
