# Blend V2 Audit + Certora Formal Verification details
- Total Audit Prize Pool: $125,000 in USDC
  - HM awards: $73,500 in USDC
  - QA awards: $2,800 in USDC
  - Judge awards: $5,000 in USDC
  - Validator awards: $3,200 in USDC 
  - Scout awards: $500 in USDC
  - Mitigation Review: $20,000 in USDC
  - Formal Verification: up to $20,000 in USDC
    - Real Bug Rules: $4,000 in USDC
    - Coverage Rules: $14,000 in USDC
    - Participation Rules: $2,000 in USDC
- [Read our guidelines for more details](https://docs.code4rena.com/roles/wardens)
- Starts February 24, 2025 20:00 UTC
- Ends March 17, 2025 20:00 UTC

**Formal Verification pool is conditional:** If no valid rules are submitted, the Formal Verification awards will be added to the HM award pool. 

ℹ️ This audit includes **deployed code,** and [the "live criticals" exception](https://docs.code4rena.com/awarding/incentive-model-and-awards#the-live-criticals-exception) therefore applies. Please see the section titled "Live/Deployed Code" for details.

**Note re: risk level upgrades/downgrades**

Two important notes about judging phase risk adjustments: 
- High- or Medium-risk submissions downgraded to Low-risk (QA) will be ineligible for awards.
- Upgrading a Low-risk finding from a QA report to a Medium- or High-risk finding is not supported.

As such, wardens are encouraged to select the appropriate risk level carefully during the submission phase.

## Live/Deployed Code

For the purposes of the ["live criticals" exception](https://docs.code4rena.com/awarding/incentive-model-and-awards#the-live-criticals-exception) and [sensitive disclosure process](https://docs.code4rena.com/roles/wardens/submission-guidelines#how-to-submit-zero-day-or-otherwise-highly-sensitive-bugs), contracts in the following directories should be considered live code, as they have only changed incrementally from the V1 code, which is deployed: 
- `./blend-contracts-v2/pool/*`
- `./blend-contracts-v2/pool-factory/*`
- `./blend-contracts-v2/backstop/*`

All other code in scope is _not_ deployed. Vulnerabilities with a root cause in any part of the codebase _except_ the above-listed directories should therefore be submitted via the standard submission form/process.

## Automated Findings / Publicly Known Issues

_Note for C4 wardens: Anything included in this `Automated Findings / Publicly Known Issues` section is considered a publicly known issue and is ineligible for awards._

Any issues that have already been uncovered here at the start of the contest are considered out-of-scope: https://github.com/blend-capital/blend-contracts-v2/issues

The following issues have been uncovered by the Blend team via dedicated issues in their [GitHub page](https://github.com/blend-capital/blend-contracts-v2/issues) and should thus be considered out-of-scope:

- [#18](https://github.com/blend-capital/blend-contracts-v2/issues/18) u64 Optimization
  - The Blend team did not notice any optimizations arising from the type adjustments
- [#11](https://github.com/blend-capital/blend-contracts-v2/issues/11) Remove "spender" and "to" from "submit"
  - The Blend team decided to not implement the relevant `interface` changes as they believe the current approach is more readable and the flash-loan implementation that relied on this change was implemented in a different way
- [#4](https://github.com/blend-capital/blend-contracts-v2/issues/4) Simplify redundent token transfers
  - The issue of redundant token transfers is known and its implementation was considered too complex to implement at this stage

Additionally, the issue outlined below contains a lot of interesting technical information around flash-loans as well as possible known risks considered:

- [#7](https://github.com/blend-capital/blend-contracts-v2/issues/7) Add flash loans 

# Overview

The `blend-contracts-v2` subfolder contains the smart contacts for an implementation of the Blend Protocol. Blend is a universal liquidity protocol primitive that enables the permissionless creation of lending pools.

The `fee-vault` subfolder represents the fee vault for Blend pools. It is used to allow an admin to collect a portion of the interest earned from blend pools by the vault depositors along with all emissions accrued by vault depositors. Wallets and integrating protocols are the entities typically interested in this functionality.

- See the [Formal Verification repo `README`](https://github.com/code-423n4/2025-02-blend-fv) for details about the Formal Verification portion of the competition.

## Links

- **Previous audits:**  https://github.com/blend-capital/blend-contracts/tree/main/audits (V1 Audits)
- **Documentation:** https://docs.blend.capital/
- **Website:** https://www.blend.capital/
- **X/Twitter:** https://x.com/blend_capital
- **Discord:** https://discord.com/invite/a6CDBQQcjW

---

# Scope

The implementations in-scope of the contest may contain unit tests defined within the code of the implementation itself. Those unit tests are clearly annotated via `#[cfg(test)]`, `#[test]`, or any other similar language-supported syntax and are considered out-of-scope for the purposes of the contest.

### Files in scope

| Files | Interfaces | nSLOC |  Libraries used |
| -------- | -------- | -------- |   -------- |
|fee-vault/src/constants.rs | **** | 3 |N/A| 
|fee-vault/src/contract.rs | **** | 147 | N/A | 
|fee-vault/src/errors.rs | **** | 15 | N/A | 
|fee-vault/src/events.rs | **** | 65 | N/A | 
|fee-vault/src/lib.rs | **** | 16 | N/A | 
|fee-vault/src/pool.rs | **** | 42 | N/A | 
|fee-vault/src/reserve_vault.rs | **** | 1178 | N/A | 
|fee-vault/src/storage.rs | **** | 150 | N/A | 
|fee-vault/src/validator.rs | **** | 12 | | N/A
|blend-contracts-v2/backstop/src/backstop/deposit.rs | **** | 203 | N/A | 
|blend-contracts-v2/backstop/src/backstop/fund_management.rs | **** | 225 | N/A | 
|blend-contracts-v2/backstop/src/backstop/mod.rs | **** | 13 | N/A | 
|blend-contracts-v2/backstop/src/backstop/pool.rs | **** | 450 | N/A | 
|blend-contracts-v2/backstop/src/backstop/user.rs | **** | 636 | N/A | 
|blend-contracts-v2/backstop/src/backstop/withdrawal.rs | **** | 403 | N/A | 
|blend-contracts-v2/backstop/src/constants.rs | **** | 6 | N/A | 
|blend-contracts-v2/backstop/src/contract.rs | **** | 150 | N/A | 
|blend-contracts-v2/backstop/src/dependencies/comet.rs | **** | 2 | N/A | 
|blend-contracts-v2/backstop/src/dependencies/mod.rs | **** | 7 | N/A | 
|blend-contracts-v2/backstop/src/dependencies/pool_factory.rs | **** | 2 | N/A | 
|blend-contracts-v2/backstop/src/emissions/claim.rs |  **** | 539 | N/A | 
|blend-contracts-v2/backstop/src/emissions/distributor.rs | **** | 608 | N/A | 
|blend-contracts-v2/backstop/src/emissions/manager.rs | **** | 2137 | N/A | 
|blend-contracts-v2/backstop/src/emissions/mod.rs | **** | 8 | N/A | 
|blend-contracts-v2/backstop/src/errors.rs | **** | 23 | N/A | 
|blend-contracts-v2/backstop/src/events.rs | **** | 67 | N/A | 
|blend-contracts-v2/backstop/src/lib.rs | **** | 16 | N/A | 
|blend-contracts-v2/backstop/src/storage.rs | **** | 349 | N/A | 
|blend-contracts-v2/pool-factory/src/errors.rs | **** | 9 | N/A | 
|blend-contracts-v2/pool-factory/src/events.rs | **** | 8 | N/A | 
|blend-contracts-v2/pool-factory/src/lib.rs | **** | 11 | N/A | 
|blend-contracts-v2/pool-factory/src/pool_factory.rs | **** | 83 | N/A | 
|blend-contracts-v2/pool-factory/src/storage.rs | **** | 58 | N/A | 
|blend-contracts-v2/pool/src/auctions/auction.rs | **** | 1771 | N/A | 
|blend-contracts-v2/pool/src/auctions/backstop_interest_auction.rs | **** | 1296 | N/A | 
|blend-contracts-v2/pool/src/auctions/bad_debt_auction.rs | **** | 2018 | N/A | 
|blend-contracts-v2/pool/src/auctions/mod.rs | **** | 5 | N/A | 
|blend-contracts-v2/pool/src/auctions/user_liquidation_auction.rs | **** | 2587 | N/A | 
|blend-contracts-v2/pool/src/constants.rs | **** | 5 | N/A |  
|blend-contracts-v2/pool/src/contract.rs | **** | 254 | N/A | 
|blend-contracts-v2/pool/src/dependencies/backstop.rs | **** | 2 | N/A | 
|blend-contracts-v2/pool/src/dependencies/mod.rs | **** | 2 | N/A | 
|blend-contracts-v2/pool/src/emissions/distributor.rs | **** | 1404 | N/A | 
|blend-contracts-v2/pool/src/emissions/manager.rs | **** | 530 | N/A | 
|blend-contracts-v2/pool/src/emissions/mod.rs | **** | 4 | N/A |  
|blend-contracts-v2/pool/src/errors.rs | **** | 37 | N/A | 
|blend-contracts-v2/pool/src/events.rs | **** | 144 | N/A | 
|blend-contracts-v2/pool/src/lib.rs | **** | 25 | N/A | 
|blend-contracts-v2/pool/src/pool/actions.rs | **** | 1718 | N/A |  
|blend-contracts-v2/pool/src/pool/bad_debt.rs | **** | 234 | N/A | 
|blend-contracts-v2/pool/src/pool/config.rs | **** | 1066 | N/A |  
|blend-contracts-v2/pool/src/pool/gulp.rs | **** | 213 |N/A |
|blend-contracts-v2/pool/src/pool/health_factor.rs | **** | 287 | N/A | 
|blend-contracts-v2/pool/src/pool/interest.rs | **** | 350 | N/A | 
|blend-contracts-v2/pool/src/pool/mod.rs | **** | 27 | N/A | 
|blend-contracts-v2/pool/src/pool/pool.rs | **** | 680 | N/A | 
|blend-contracts-v2/pool/src/pool/reserve.rs | **** | 589 | N/A | 
|blend-contracts-v2/pool/src/pool/status.rs | **** | 956 | N/A | 
|blend-contracts-v2/pool/src/pool/submit.rs | **** | 1863 | N/A | 
|blend-contracts-v2/pool/src/pool/user.rs | **** | 1004 | N/A | 
|blend-contracts-v2/pool/src/storage.rs | **** | 380 | N/A | 
|blend-contracts-v2/pool/src/validator.rs | **** | 7 | N/A | 
|Totals|  | 27099 |  | 

### Files out of scope

Any file that is not explicitly included in the aforementioned list is to be considered out-of-scope.

## Scoping Q &amp; A

| Question                                | Answer                       |
| --------------------------------------- | ---------------------------- |
| ERC20 used by the protocol              |       [Stellar Asset Contracts (SACs)](https://developers.stellar.org/docs/tokens/token-interface) & Standard [SEP-41](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md) Soroban Tokens             |
| Test coverage                           | N/A                          |
| ERC721 used  by the protocol            |            N/A              |
| ERC777 used by the protocol             |           N/A                |
| ERC1155 used by the protocol            |              N/A           |
| Chains the protocol will be deployed on | Stellar Network  |

### ERC20 token behaviors in scope

| Question                                                                                                                                                   | Answer |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| [Missing return values](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#missing-return-values)                                                      |   Out of scope  |
| [Fee on transfer](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#fee-on-transfer)                                                                  |  Out of scope  |
| [Balance changes outside of transfers](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#balance-modifications-outside-of-transfers-rebasingairdrops) | Out of scope    |
| [Upgradeability](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#upgradable-tokens)                                                                 |   Out of scope  |
| [Flash minting](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#flash-mintable-tokens)                                                              | Out of scope    |
| [Pausability](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#pausable-tokens)                                                                      | Out of scope    |
| [Approval race protections](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#approval-race-protections)                                              | Out of scope    |
| [Revert on approval to zero address](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-approval-to-zero-address)                            | Out of scope    |
| [Revert on zero value approvals](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-approvals)                                    | Out of scope    |
| [Revert on zero value transfers](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-transfers)                                    | Out of scope    |
| [Revert on transfer to the zero address](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-transfer-to-the-zero-address)                    | Out of scope    |
| [Revert on large approvals and/or transfers](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-large-approvals--transfers)                  | Out of scope    |
| [Doesn't revert on failure](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#no-revert-on-failure)                                                   |  Out of scope   |
| [Multiple token addresses](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-transfers)                                          | Out of scope    |
| [Low decimals ( < 6)](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#low-decimals)                                                                 |   Out of scope  |
| [High decimals ( > 18)](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#high-decimals)                                                              | Out of scope    |
| [Blocklists](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#tokens-with-blocklists)                                                                | Out of scope    |

### External integrations (e.g., Uniswap) behavior in scope:


| Question                                                  | Answer |
| --------------------------------------------------------- | ------ |
| Enabling/disabling fees (e.g. Blur disables/enables fees) | No   |
| Pausability (e.g. Uniswap pool gets paused)               |  Yes   |
| Upgradeability (e.g. Uniswap gets upgraded)               |   No  |


### EIP compliance checklist

N/A

# Additional context

## Main invariants

* User's cannot extract funds (borrow, withdraw) from a pool if they do not meet or exceed the minimum health factor

## Attack ideas (where to focus for bugs)

### Auctions

The protocol conducts auctions to both process liquidations and pay out backstop interest. Anything that can disrupt / block / break the creation or filling of these auctions has potential to impact the health of a pool and/or it's backstop.

The auction creation system was modified in v2 to allow auction creators to specify the assets in the bid/lot. https://github.com/blend-capital/blend-contracts-v2/issues/3

### Flash Loans

Blend v2 introduces a flash loans endpoint. These are slightly different than EVM based flash loans, but still allow external contracts to be invoked. with borrowed funds, and repaid afterwards, or remain borrowed with the appropriate amount of collateral.

## All trusted roles in the protocol

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Admin                          | Able to add reserves and edit reserve/pool configurations (excluding oracle) |

## Describe any novel or unique curve logic or mathematical models implemented in the contracts:

Interest uses a capped integral controller to adjust the interest rates to help utilization reach the target utilization rate: https://docs.blend.capital/blend-whitepaper#interest-rates

## Running tests

**Building**

[Soroban Setup](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup) Prerequisites

```bash 
# configure relevant rust target 
rustup target add wasm32-unknown-unknown

# install stellar cli
cargo install --locked stellar-cli@22.2.0 --features opt
```

To compile project (in either folder)

```bash 
make
```

To run tests (in either folder)

```bash 
make test
```

## Miscellaneous

Employees of Blend and employees' family members are ineligible to participate in this audit.

Code4rena's rules cannot be overridden by the contents of this README. In case of doubt, please check with C4 staff.
