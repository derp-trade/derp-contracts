use anchor_lang::prelude::*;

pub fn get_pyth_price(pyth_price_account: &AccountInfo) -> Result<u64> {
    // Load the price feed from the account
    let price_feed = load_price_feed_from_account_info(pyth_price_account)
        .map_err(|_| error!(DErrorCode::InvalidOracleAccount))?;
    
    // Get the current price
    let price = price_feed.get_current_price()
        .ok_or(error!(DErrorCode::InvalidOraclePrice))?;
    
    // Check if the price is valid
    require!(
        price.status == PriceStatus::Trading,
        DErrorCode::InvalidOraclePrice
    );
    
    // Convert price to a standard format (e.g., USD with 6 decimals)
    // The price is represented as a fixed-point number with 'expo' number of decimal places
    // We want to convert it to a u64 with 6 decimal places
    
    // First, get the price as a signed integer
    let price_value = price.price;
    
    // Get the exponent (negative for decimal places)
    let expo = price.expo;
    
    // Convert to a standard format with 6 decimal places
    let normalized_price = if expo <= -6 {
        // If expo is already more negative than -6, we need to divide
        // For example, if expo is -8, we divide by 10^(8-6) = 10^2
        let divisor = 10_i64.pow((expo.abs() - 6) as u32);
        (price_value / divisor) as u64
    } else {
        // If expo is less negative than -6 or positive, we need to multiply
        // For example, if expo is -4, we multiply by 10^(6-4) = 10^2
        let multiplier = 10_i64.pow((6 - expo.abs()) as u32);
        (price_value * multiplier) as u64
    };
    
    // Handle negative prices (though they should be rare in most use cases)
    if normalized_price < 0 {
        msg!("Warning: Negative price detected from Pyth oracle");
        return Err(error!(DErrorCode::InvalidOraclePrice));
    }
    
    // Log the price information for debugging
    msg!(
        "Pyth price: raw={}, expo={}, normalized={}",
        price_value,
        expo,
        normalized_price
    );
    
    Ok(normalized_price as u64)
}

// Additional helper function to get price with confidence interval
pub fn get_pyth_price_with_confidence(pyth_price_account: &AccountInfo) -> Result<(u64, u64)> {
    // Load the price feed from the account
    let price_feed = load_price_feed_from_account_info(pyth_price_account)
        .map_err(|_| error!(DErrorCode::InvalidOracleAccount))?;
    
    // Get the current price
    let price = price_feed.get_current_price()
        .ok_or(error!(DErrorCode::InvalidOraclePrice))?;
    
    // Check if the price is valid
    require!(
        price.status == PriceStatus::Trading,
        DErrorCode::InvalidOraclePrice
    );
    
    // Get the confidence interval
    let conf = price.conf;
    
    // Convert price to a standard format (e.g., USD with 6 decimals)
    let expo = price.expo;
    let price_value = price.price;
    
    // Convert to a standard format with 6 decimal places
    let normalized_price = if expo <= -6 {
        (price_value / 10_i64.pow((expo.abs() - 6) as u32)) as u64
    } else {
        (price_value * 10_i64.pow((6 - expo.abs()) as u32)) as u64
    };
    
    // Convert confidence to the same scale
    let normalized_conf = if expo <= -6 {
        (conf as i64 / 10_i64.pow((expo.abs() - 6) as u32)) as u64
    } else {
        (conf as i64 * 10_i64.pow((6 - expo.abs()) as u32)) as u64
    };
    
    Ok((normalized_price as u64, normalized_conf))
}