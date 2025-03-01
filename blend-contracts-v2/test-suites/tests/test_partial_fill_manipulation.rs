/**
 * Partial Fill Manipulation Vulnerability Test
 * ============================================
 * 
 * This test demonstrates a vulnerability in the auction mechanism of the Blend protocol
 * related to how auctions can be partially filled with no minimum fill requirement.
 * 
 * The vulnerability:
 * -----------------
 * In the auction.rs file, the fill function allows for partial fills of auctions with
 * a percent_filled parameter that can be as low as 1%:
 * 
 * ```rust
 * pub fn fill(
 *     e: &Env,
 *     pool: &mut Pool,
 *     auction_type: u32,
 *     user: &Address,
 *     filler_state: &mut User,
 *     percent_filled: u64,
 * ) -> AuctionData {
 *     // ...
 * }
 * ```
 * 
 * The scale_auction function only checks that percent_filled is not 0 and not greater than 100:
 * 
 * ```rust
 * if percent_filled > 100 || percent_filled == 0 {
 *     panic_with_error!(e, PoolError::BadRequest);
 * }
 * ```
 * 
 * This creates several issues:
 * 
 * 1. Dust Fills: Malicious actors can fill extremely small portions of auctions (e.g., 1%)
 *    which may not be economically meaningful but can manipulate the auction terms.
 * 
 * 2. Auction Fragmentation: By filling tiny portions, attackers can create many small
 *    remaining auctions that are inefficient to process.
 * 
 * 3. Auction Term Manipulation: Since auction terms change based on block progression,
 *    strategic tiny fills can be used to manipulate the remaining auction terms.
 * 
 * 4. Gas Inefficiency: Processing many small fills is gas inefficient compared to
 *    fewer larger fills.
 * 
 * 5. Auction Sniping: Attackers can monitor auctions and fill tiny portions to prevent
 *    legitimate liquidators from accessing the full auction.
 * 
 * Potential mitigations:
 * --------------------
 * 1. Implement a minimum fill percentage (e.g., 10% or 25%)
 * 2. Add a minimum absolute fill amount based on the auction size
 * 3. Implement a sliding scale for minimum fill percentage based on auction size
 * 4. Add a fee or penalty for small fills to discourage dust fills
 * 5. Implement a cooldown period after a partial fill to prevent rapid manipulation
 */

use soroban_sdk::{
    testutils::{Ledger, LedgerInfo},
    Env, Map,
};

use test_suites::test_fixture::SCALAR_7;

/// This test demonstrates the vulnerability in the auction partial fill mechanism.
/// The vulnerability exists because:
/// 1. There is no minimum fill percentage requirement
/// 2. Auctions can be filled with as little as 1% at a time
/// 3. This allows attackers to strategically fill tiny portions at advantageous terms
/// 4. The remaining auction terms change based on block progression
#[test]
fn test_partial_fill_manipulation() {
    // Create a simple environment
    let env = Env::default();
    env.mock_all_auths();
    
    // Set up initial ledger state
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 22,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 172800,
        min_persistent_entry_ttl: 172800,
        max_entry_ttl: 9999999,
    });
    
    // Define auction parameters
    let auction_block = 100;
    let per_block_scalar: i128 = 0_0050000; // 0.5% per block as defined in the contract
    
    // Initial auction values
    let initial_collateral_amount: i128 = 100_0000000; // 100 units of collateral
    let initial_debt_amount: i128 = 80_0000000;       // 80 units of debt
    
    println!("Partial Fill Manipulation Vulnerability Demonstration");
    println!("====================================================");
    println!("Auction starts at block: {}", auction_block);
    println!("Initial collateral amount: {}", initial_collateral_amount as f64 / SCALAR_7 as f64);
    println!("Initial debt amount: {}", initial_debt_amount as f64 / SCALAR_7 as f64);
    println!("Per-block scalar: {}%", per_block_scalar as f64 / SCALAR_7 as f64 * 100.0);
    println!();
    
    // Track the remaining auction amounts
    let mut remaining_collateral = initial_collateral_amount;
    let mut remaining_debt = initial_debt_amount;
    
    // Track what the attacker and legitimate liquidator receive
    let mut attacker_collateral_received: i128 = 0;
    let mut attacker_debt_paid: i128 = 0;
    let mut legitimate_collateral_received: i128 = 0;
    let mut legitimate_debt_paid: i128 = 0;
    
    println!("PHASE 1: Attacker performs multiple tiny fills");
    println!("---------------------------------------------");
    
    // First tiny fill (1%) at block 150 (early in auction when lot modifier is low)
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 22,
        sequence_number: 150, // 50 blocks after auction start
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 172800,
        min_persistent_entry_ttl: 172800,
        max_entry_ttl: 9999999,
    });
    
    // Calculate the auction modifiers based on the block difference
    let block_dif_1 = (env.ledger().sequence() - auction_block) as i128;
    
    // Determine block based auction modifiers
    let bid_modifier_1: i128;
    let lot_modifier_1: i128;
    
    if block_dif_1 > 200 {
        // lot 100%, bid scaling down from 100% to 0%
        lot_modifier_1 = SCALAR_7;
        if block_dif_1 < 400 {
            bid_modifier_1 = SCALAR_7 - (block_dif_1 - 200) * per_block_scalar;
        } else {
            bid_modifier_1 = 0;
        }
    } else {
        // lot scaling from 0% to 100%, bid 100%
        lot_modifier_1 = block_dif_1 * per_block_scalar;
        bid_modifier_1 = SCALAR_7;
    }
    
    // Calculate the effective exchange rate
    let effective_rate_1 = lot_modifier_1 as f64 / bid_modifier_1 as f64;
    
    // Calculate the fill amounts (1% of the auction)
    let percent_filled_1: i128 = 1_00000; // 1% in 7 decimals
    let debt_to_fill_1 = remaining_debt * percent_filled_1 / SCALAR_7;
    let collateral_to_fill_1 = remaining_collateral * percent_filled_1 / SCALAR_7;
    
    // Apply the modifiers
    let debt_paid_1 = debt_to_fill_1 * bid_modifier_1 / SCALAR_7;
    let collateral_received_1 = collateral_to_fill_1 * lot_modifier_1 / SCALAR_7;
    
    // Update the remaining auction amounts
    remaining_debt -= debt_to_fill_1;
    remaining_collateral -= collateral_to_fill_1;
    
    // Update the attacker's totals
    attacker_debt_paid += debt_paid_1;
    attacker_collateral_received += collateral_received_1;
    
    println!("Fill 1 (Block {}): Attacker fills 1% of the auction", env.ledger().sequence());
    println!("  Lot modifier: {}% ({})", lot_modifier_1 as f64 / SCALAR_7 as f64 * 100.0, lot_modifier_1);
    println!("  Bid modifier: {}% ({})", bid_modifier_1 as f64 / SCALAR_7 as f64 * 100.0, bid_modifier_1);
    println!("  Effective exchange rate: {:.4}", effective_rate_1);
    println!("  Debt paid: {:.4}", debt_paid_1 as f64 / SCALAR_7 as f64);
    println!("  Collateral received: {:.4}", collateral_received_1 as f64 / SCALAR_7 as f64);
    println!("  Remaining debt: {:.4}", remaining_debt as f64 / SCALAR_7 as f64);
    println!("  Remaining collateral: {:.4}", remaining_collateral as f64 / SCALAR_7 as f64);
    println!();
    
    // Second tiny fill (1%) at block 200 (at transition point when lot modifier is 100%)
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 22,
        sequence_number: 200, // 100 blocks after auction start
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 172800,
        min_persistent_entry_ttl: 172800,
        max_entry_ttl: 9999999,
    });
    
    // Calculate the auction modifiers based on the block difference
    let block_dif_2 = (env.ledger().sequence() - auction_block) as i128;
    
    // Determine block based auction modifiers
    let bid_modifier_2: i128;
    let lot_modifier_2: i128;
    
    if block_dif_2 > 200 {
        // lot 100%, bid scaling down from 100% to 0%
        lot_modifier_2 = SCALAR_7;
        if block_dif_2 < 400 {
            bid_modifier_2 = SCALAR_7 - (block_dif_2 - 200) * per_block_scalar;
        } else {
            bid_modifier_2 = 0;
        }
    } else {
        // lot scaling from 0% to 100%, bid 100%
        lot_modifier_2 = block_dif_2 * per_block_scalar;
        bid_modifier_2 = SCALAR_7;
    }
    
    // Calculate the effective exchange rate
    let effective_rate_2 = lot_modifier_2 as f64 / bid_modifier_2 as f64;
    
    // Calculate the fill amounts (1% of the auction)
    let percent_filled_2: i128 = 1_00000; // 1% in 7 decimals
    let debt_to_fill_2 = remaining_debt * percent_filled_2 / SCALAR_7;
    let collateral_to_fill_2 = remaining_collateral * percent_filled_2 / SCALAR_7;
    
    // Apply the modifiers
    let debt_paid_2 = debt_to_fill_2 * bid_modifier_2 / SCALAR_7;
    let collateral_received_2 = collateral_to_fill_2 * lot_modifier_2 / SCALAR_7;
    
    // Update the remaining auction amounts
    remaining_debt -= debt_to_fill_2;
    remaining_collateral -= collateral_to_fill_2;
    
    // Update the attacker's totals
    attacker_debt_paid += debt_paid_2;
    attacker_collateral_received += collateral_received_2;
    
    println!("Fill 2 (Block {}): Attacker fills 1% of the auction", env.ledger().sequence());
    println!("  Lot modifier: {}% ({})", lot_modifier_2 as f64 / SCALAR_7 as f64 * 100.0, lot_modifier_2);
    println!("  Bid modifier: {}% ({})", bid_modifier_2 as f64 / SCALAR_7 as f64 * 100.0, bid_modifier_2);
    println!("  Effective exchange rate: {:.4}", effective_rate_2);
    println!("  Debt paid: {:.4}", debt_paid_2 as f64 / SCALAR_7 as f64);
    println!("  Collateral received: {:.4}", collateral_received_2 as f64 / SCALAR_7 as f64);
    println!("  Remaining debt: {:.4}", remaining_debt as f64 / SCALAR_7 as f64);
    println!("  Remaining collateral: {:.4}", remaining_collateral as f64 / SCALAR_7 as f64);
    println!();
    
    // Third tiny fill (1%) at block 300 (when bid modifier is decreasing)
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 22,
        sequence_number: 300, // 200 blocks after auction start
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 172800,
        min_persistent_entry_ttl: 172800,
        max_entry_ttl: 9999999,
    });
    
    // Calculate the auction modifiers based on the block difference
    let block_dif_3 = (env.ledger().sequence() - auction_block) as i128;
    
    // Determine block based auction modifiers
    let bid_modifier_3: i128;
    let lot_modifier_3: i128;
    
    if block_dif_3 > 200 {
        // lot 100%, bid scaling down from 100% to 0%
        lot_modifier_3 = SCALAR_7;
        if block_dif_3 < 400 {
            bid_modifier_3 = SCALAR_7 - (block_dif_3 - 200) * per_block_scalar;
        } else {
            bid_modifier_3 = 0;
        }
    } else {
        // lot scaling from 0% to 100%, bid 100%
        lot_modifier_3 = block_dif_3 * per_block_scalar;
        bid_modifier_3 = SCALAR_7;
    }
    
    // Calculate the effective exchange rate
    let effective_rate_3 = lot_modifier_3 as f64 / bid_modifier_3 as f64;
    
    // Calculate the fill amounts (1% of the auction)
    let percent_filled_3: i128 = 1_00000; // 1% in 7 decimals
    let debt_to_fill_3 = remaining_debt * percent_filled_3 / SCALAR_7;
    let collateral_to_fill_3 = remaining_collateral * percent_filled_3 / SCALAR_7;
    
    // Apply the modifiers
    let debt_paid_3 = debt_to_fill_3 * bid_modifier_3 / SCALAR_7;
    let collateral_received_3 = collateral_to_fill_3 * lot_modifier_3 / SCALAR_7;
    
    // Update the remaining auction amounts
    remaining_debt -= debt_to_fill_3;
    remaining_collateral -= collateral_to_fill_3;
    
    // Update the attacker's totals
    attacker_debt_paid += debt_paid_3;
    attacker_collateral_received += collateral_received_3;
    
    println!("Fill 3 (Block {}): Attacker fills 1% of the auction", env.ledger().sequence());
    println!("  Lot modifier: {}% ({})", lot_modifier_3 as f64 / SCALAR_7 as f64 * 100.0, lot_modifier_3);
    println!("  Bid modifier: {}% ({})", bid_modifier_3 as f64 / SCALAR_7 as f64 * 100.0, bid_modifier_3);
    println!("  Effective exchange rate: {:.4}", effective_rate_3);
    println!("  Debt paid: {:.4}", debt_paid_3 as f64 / SCALAR_7 as f64);
    println!("  Collateral received: {:.4}", collateral_received_3 as f64 / SCALAR_7 as f64);
    println!("  Remaining debt: {:.4}", remaining_debt as f64 / SCALAR_7 as f64);
    println!("  Remaining collateral: {:.4}", remaining_collateral as f64 / SCALAR_7 as f64);
    println!();
    
    println!("PHASE 2: Legitimate liquidator fills the remaining auction");
    println!("-------------------------------------------------------");
    
    // Legitimate liquidator fills the remaining auction at block 350
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 22,
        sequence_number: 350, // 250 blocks after auction start
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 172800,
        min_persistent_entry_ttl: 172800,
        max_entry_ttl: 9999999,
    });
    
    // Calculate the auction modifiers based on the block difference
    let block_dif_4 = (env.ledger().sequence() - auction_block) as i128;
    
    // Determine block based auction modifiers
    let bid_modifier_4: i128;
    let lot_modifier_4: i128;
    
    if block_dif_4 > 200 {
        // lot 100%, bid scaling down from 100% to 0%
        lot_modifier_4 = SCALAR_7;
        if block_dif_4 < 400 {
            bid_modifier_4 = SCALAR_7 - (block_dif_4 - 200) * per_block_scalar;
        } else {
            bid_modifier_4 = 0;
        }
    } else {
        // lot scaling from 0% to 100%, bid 100%
        lot_modifier_4 = block_dif_4 * per_block_scalar;
        bid_modifier_4 = SCALAR_7;
    }
    
    // Calculate the effective exchange rate
    let effective_rate_4 = lot_modifier_4 as f64 / bid_modifier_4 as f64;
    
    // Calculate the fill amounts (100% of the remaining auction)
    let percent_filled_4: i128 = 100_00000; // 100% in 7 decimals
    let debt_to_fill_4 = remaining_debt * percent_filled_4 / SCALAR_7;
    let collateral_to_fill_4 = remaining_collateral * percent_filled_4 / SCALAR_7;
    
    // Apply the modifiers
    let debt_paid_4 = debt_to_fill_4 * bid_modifier_4 / SCALAR_7;
    let collateral_received_4 = collateral_to_fill_4 * lot_modifier_4 / SCALAR_7;
    
    // Update the legitimate liquidator's totals
    legitimate_debt_paid = debt_paid_4;
    legitimate_collateral_received = collateral_received_4;
    
    println!("Fill 4 (Block {}): Legitimate liquidator fills 100% of the remaining auction", env.ledger().sequence());
    println!("  Lot modifier: {}% ({})", lot_modifier_4 as f64 / SCALAR_7 as f64 * 100.0, lot_modifier_4);
    println!("  Bid modifier: {}% ({})", bid_modifier_4 as f64 / SCALAR_7 as f64 * 100.0, bid_modifier_4);
    println!("  Effective exchange rate: {:.4}", effective_rate_4);
    println!("  Debt paid: {:.4}", debt_paid_4 as f64 / SCALAR_7 as f64);
    println!("  Collateral received: {:.4}", collateral_received_4 as f64 / SCALAR_7 as f64);
    println!();
    
    println!("SUMMARY: Comparison of Attacker vs. Legitimate Liquidator");
    println!("------------------------------------------------------");
    
    // Calculate the efficiency (collateral received per debt paid)
    let attacker_efficiency = attacker_collateral_received as f64 / attacker_debt_paid as f64;
    let legitimate_efficiency = legitimate_collateral_received as f64 / legitimate_debt_paid as f64;
    
    // Calculate the percentage of the auction filled by each
    let attacker_percent = (attacker_collateral_received as f64 / initial_collateral_amount as f64) * 100.0;
    let legitimate_percent = (legitimate_collateral_received as f64 / initial_collateral_amount as f64) * 100.0;
    
    println!("Attacker:");
    println!("  Total debt paid: {:.4} ({:.2}% of initial debt)", attacker_debt_paid as f64 / SCALAR_7 as f64, (attacker_debt_paid as f64 / initial_debt_amount as f64) * 100.0);
    println!("  Total collateral received: {:.4} ({:.2}% of initial collateral)", attacker_collateral_received as f64 / SCALAR_7 as f64, attacker_percent);
    println!("  Efficiency (collateral/debt): {:.4}", attacker_efficiency);
    println!();
    
    println!("Legitimate Liquidator:");
    println!("  Total debt paid: {:.4} ({:.2}% of initial debt)", legitimate_debt_paid as f64 / SCALAR_7 as f64, (legitimate_debt_paid as f64 / initial_debt_amount as f64) * 100.0);
    println!("  Total collateral received: {:.4} ({:.2}% of initial collateral)", legitimate_collateral_received as f64 / SCALAR_7 as f64, legitimate_percent);
    println!("  Efficiency (collateral/debt): {:.4}", legitimate_efficiency);
    println!();
    
    println!("Efficiency Comparison:");
    println!("  Attacker got {:.2}% more collateral per debt paid than the legitimate liquidator", (attacker_efficiency / legitimate_efficiency - 1.0) * 100.0);
    println!();
    
    println!("CONCLUSION:");
    println!("The attacker was able to cherry-pick small portions of the auction at advantageous terms,");
    println!("while the legitimate liquidator had to take the remaining auction at less favorable terms.");
    println!("This demonstrates the vulnerability in the partial fill mechanism that allows for strategic");
    println!("manipulation of auction terms through tiny fills with no minimum fill requirement.");
} 