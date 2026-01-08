# Solana Lending Program Development Challenge Tester

This is a program that validates your progress on the "Solana Lending Program
Development" challenge using the Mollusk test harness.

## Overview

The tester has been enhanced with the Mollusk framework, a lightweight test
harness for Solana programs. This provides:

- **Fast Execution**: Direct SVM instruction execution without full validator runtime
- **Precise Control**: Explicit account management and instruction validation
- **Better Testing**: Comprehensive checks for account state, lamports, data, and more

## Architecture

The tester is organized into the following modules:

- `src/main.rs` - Entry point that initializes the tester
- `src/definition.rs` - Test case definitions
- `src/stages/` - Test implementations organized by stage
- `src/mollusk/` - Mollusk integration module
  - `mod.rs` - Main Mollusk management
  - `program_loader.rs` - Program loading utilities
  - `test_context.rs` - Test context for state management
- `src/helpers.rs` - Helper functions for testing

## Requirements for binary

- Following environment variables:
  - `STACKCLASS_REPOSITORY_DIR` - root of the user's code submission
  - `STACKCLASS_TEST_CASES_JSON` - test cases in JSON format

## User code requirements

- A binary named `your_program.sh` that executes the program.
- A file named `stackclass.yml`, with the following values: `debug`.

## Building

```bash
cargo build --release
```

## Running Tests

The tester will automatically load and test the user's lending program using
Mollusk. Ensure the program is built before running tests:

```bash
cd <user-repo>
anchor build
```

## Test Stages

### Base Stages (7)
1. **be1** - Environment Setup
2. **rs2** - Rust Basics
3. **sm3** - Solana Model
4. **at4** - Anchor Try
5. **st5** - SPL Token Basics
6. **cp6** - Basic Deposit
7. **tt7** - Basic Withdraw

### Extension Modules (8 modules × 4 stages = 32 cases)

#### PDA Module
- **pa1** - PDA Concept
- **pa2** - PDA Derivation
- **pa3** - Bump Seeds
- **pa4** - PDA Practice

#### Treasury Module
- **tr1** - Treasury Intro
- **tr2** - Treasury Creation
- **tr3** - Treasury Security
- **tr4** - Treasury Practice

#### Account Structure Module
- **as1** - Bank Account
- **as2** - User Account
- **as3** - Account Space
- **as4** - Account Practice

#### Lending Core Module
- **lc1** - Borrow Basics
- **lc2** - Repay Basics
- **lc3** - LTV Calculation
- **lc4** - Core Practice

#### Oracle Module
- **or1** - Oracle Concept
- **or2** - Pyth Integration
- **or3** - Price Fetching
- **or4** - Oracle Practice

#### Liquidation Module
- **li1** - Health Factor
- **li2** - Liquidation Trigger
- **li3** - Liquidation Process
- **li4** - Liquidation Bonus
- **li5** - Liquidation Practice

#### Interest Module
- **in1** - Interest Basics
- **in2** - Accrued Interest
- **in3** - Rate Models
- **in4** - Interest Practice

#### Security Module
- **se1** - Common Vulnerabilities
- **se2** - Reentrancy Protection
- **se3** - Account Validation
- **se4** - Security Practice

## Mollusk Integration

The tester uses Mollusk for efficient program testing:

```rust
use crate::mollusk::{create_lending_mollusk, LendingTestContext};

// Create Mollusk instance
let mollusk = create_lending_mollusk(&repo_dir)?;
let mut context = LendingTestContext::new(mollusk)?;

// Create accounts and execute instructions
let user = context.create_funded_account(1_000_000_000);
let instruction = create_lending_instruction(data, accounts);
context.execute_instruction(&instruction)?;
```

## License

See LICENSE file for details.