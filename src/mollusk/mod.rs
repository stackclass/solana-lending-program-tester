// Copyright (c) The StackClass Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Mollusk management module for the lending program tester.
//!
//! This module provides a wrapper around the Mollusk test harness to simplify
//! testing of the lending program. It handles program loading, account setup,
//! and instruction execution.

pub mod program_loader;
pub mod test_context;

pub use program_loader::{ProgramLoadError, load_lending_program, load_lending_program_id};
pub use test_context::{LendingTestContext, TestContextError};

use mollusk_svm::Mollusk;
use solana_pubkey::Pubkey;
use std::path::Path;

/// The default program ID for the lending program.
/// This should match the program ID defined in the user's Anchor.toml.
#[allow(dead_code)]
pub const LENDING_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0x9a, 0x5e, 0x9e, 0x6a, 0x3c, 0x6b, 0x8e, 0x7d, 0x4a, 0x2f, 0x5b, 0x1c, 0x8d, 0x3e, 0x7f, 0x6a,
    0x2b, 0x8c, 0x1d, 0x4e, 0x5f, 0x6a, 0x7b, 0x8c, 0x9d, 0x0e, 0x1f, 0x2a, 0x3b, 0x4c, 0x5d, 0x6e,
]);

/// Create a new Mollusk instance for testing the lending program.
///
/// This function attempts to load the compiled lending program from the
/// user's repository directory and creates a Mollusk instance configured
/// for testing.
///
/// # Arguments
///
/// * `repo_dir` - Path to the user's repository directory
///
/// # Returns
///
/// * `Ok(Mollusk)` - A configured Mollusk instance
/// * `Err(ProgramLoadError)` - If the program cannot be loaded
pub fn create_lending_mollusk(
    repo_dir: &Path,
    program_id: &Pubkey,
) -> Result<Mollusk, ProgramLoadError> {
    let program_path = load_lending_program(repo_dir)?;
    let program_name =
        program_path.file_stem().and_then(|stem| stem.to_str()).unwrap_or("lending_program");

    let program_dir = program_path
        .parent()
        .ok_or_else(|| ProgramLoadError::ProgramDirNotFound(program_path.clone()))?;

    // SAFETY: set_var is process-global; we set it once before loading the ELF.
    unsafe {
        std::env::set_var("SBF_OUT_DIR", program_dir);
    }

    let mut mollusk = Mollusk::new(program_id, program_name);

    // Add necessary programs for testing
    add_required_programs(&mut mollusk);

    Ok(mollusk)
}

/// Add required programs to the Mollusk instance.
///
/// This includes system programs and SPL Token programs that are commonly
/// used in lending operations.
fn add_required_programs(mollusk: &mut Mollusk) {
    // System program is already included by default in Mollusk

    // SPL Token program and Associated Token program - needed for token operations
    mollusk_svm_programs_token::token::add_program(mollusk);
    mollusk_svm_programs_token::associated_token::add_program(mollusk);
}

/// Initialize a test context with the lending program.
///
/// This is a convenience function that creates both a Mollusk instance and
/// a test context for easier testing.
///
/// # Arguments
///
/// * `repo_dir` - Path to the user's repository directory
///
/// # Returns
///
/// * `Ok(LendingTestContext)` - A configured test context
/// * `Err(TestContextError)` - If initialization fails
pub fn init_test_context(repo_dir: &Path) -> Result<LendingTestContext, TestContextError> {
    let program_id = load_lending_program_id(repo_dir)?;
    let mollusk = create_lending_mollusk(repo_dir, &program_id)?;
    LendingTestContext::new(mollusk, program_id)
}
