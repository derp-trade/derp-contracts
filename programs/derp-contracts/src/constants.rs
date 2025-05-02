// Initial balance for new users ($10,000 with 6 decimal places)
pub const INITIAL_BALANCE: u64 = 10_000_000_000;

// Initial and maintenance margin requirements (as percentages with 4 decimal places)
pub const INITIAL_MARGIN_REQUIREMENT: u64 = 1_000; // 10% (0.1 * 10000)
pub const MAINTENANCE_MARGIN_REQUIREMENT: u64 = 500; // 5% (0.05 * 10000)

// Maximum leverage allowed
pub const MAX_LEVERAGE: u8 = 10;