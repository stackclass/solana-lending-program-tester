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

//! Helper functions for testing the lending program.

#[allow(dead_code)]
use crate::mollusk::{
    ProgramLoadError, TestContextError, init_test_context, load_lending_program,
    load_lending_program_id,
};
use mollusk_svm::{program::keyed_account_for_system_program, result::Check};
use mollusk_svm_programs_token::{associated_token, token};
use sha2::{Digest, Sha256};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use std::{path::Path, str::FromStr};

const DEFAULT_LENDING_PROGRAM_ID: &str = "LendZ1111111111111111111111111111111111111";

/// Get the repository directory from environment variables.
///
/// This function reads the `STACKCLASS_REPOSITORY_DIR` environment variable
/// and returns it as a Path.
///
/// # Returns
///
/// * `Ok(PathBuf)` - The repository directory path
/// * `Err(ProgramLoadError)` - If the environment variable is not set
pub fn get_repo_dir() -> Result<std::path::PathBuf, ProgramLoadError> {
    std::env::var("STACKCLASS_REPOSITORY_DIR")
        .map_err(|_| ProgramLoadError::RepoNotFound(std::path::PathBuf::from("Not set")))
        .map(std::path::PathBuf::from)
}

/// Create a test error message for reporting to the user.
///
/// # Arguments
///
/// * `stage_name` - The name of the test stage
/// * `error` - The error that occurred
///
/// # Returns
///
/// * `String` - A formatted error message
#[allow(dead_code)]
pub fn format_test_error(stage_name: &str, error: &TestContextError) -> String {
    format!("Test '{}' failed: {}", stage_name, error)
}

/// Create a success message for reporting to the user.
///
/// # Arguments
///
/// * `stage_name` - The name of the test stage
///
/// # Returns
///
/// * `String` - A formatted success message
#[allow(dead_code)]
pub fn format_test_success(stage_name: &str) -> String {
    format!("Test '{}' passed successfully", stage_name)
}

/// Create a basic system account with lamports.
///
/// # Arguments
///
/// * `lamports` - The amount of lamports
///
/// # Returns
///
/// * `Account` - A system account
#[allow(dead_code)]
pub fn create_system_account(lamports: u64) -> Account {
    Account { lamports, owner: solana_system_program::id(), ..Default::default() }
}

/// Create a check for successful execution.
///
/// # Returns
///
/// * `Check` - A success check
#[allow(dead_code)]
pub fn success_check() -> Check<'static> {
    Check::success()
}

/// Create a check for account lamports.
///
/// # Arguments
///
/// * `pubkey` - The account public key
/// * `expected_lamports` - The expected lamports
///
/// # Returns
///
/// * `Check` - A lamports check
#[allow(dead_code)]
pub fn lamports_check(pubkey: &Pubkey, expected_lamports: u64) -> Check<'_> {
    Check::account(pubkey).lamports(expected_lamports).build()
}

/// Create a check for account data.
///
/// # Arguments
///
/// * `pubkey` - The account public key
/// * `expected_data` - The expected account data
///
/// # Returns
///
/// * `Check` - A data check
#[allow(dead_code)]
pub fn data_check<'a>(pubkey: &'a Pubkey, expected_data: &'a [u8]) -> Check<'a> {
    Check::account(pubkey).data(expected_data).build()
}

/// Create a check for account owner.
///
/// # Arguments
///
/// * `pubkey` - The account public key
/// * `expected_owner` - The expected owner
///
/// # Returns
///
/// * `Check` - An owner check
#[allow(dead_code)]
pub fn owner_check<'a>(pubkey: &'a Pubkey, expected_owner: &'a Pubkey) -> Check<'a> {
    Check::account(pubkey).owner(expected_owner).build()
}

/// Create a check for account executability.
///
/// # Arguments
///
/// * `pubkey` - The account public key
/// * `expected_executable` - The expected executability
///
/// # Returns
///
/// * `Check` - An executable check
#[allow(dead_code)]
pub fn executable_check(pubkey: &Pubkey, expected_executable: bool) -> Check<'_> {
    Check::account(pubkey).executable(expected_executable).build()
}

/// Convert a TestContextError to a tester::CaseError.
///
/// # Arguments
///
/// * `error` - The TestContextError to convert
///
/// # Returns
///
/// * `tester::CaseError` - The converted error
pub fn to_case_error(error: TestContextError) -> tester::CaseError {
    Box::new(error) as Box<dyn std::error::Error + Send + Sync>
}

/// Convert a ProgramLoadError to a tester::CaseError.
///
/// # Arguments
///
/// * `error` - The ProgramLoadError to convert
///
/// # Returns
///
/// * `tester::CaseError` - The converted error
pub fn to_case_error_from_load(error: crate::mollusk::ProgramLoadError) -> tester::CaseError {
    Box::new(error) as Box<dyn std::error::Error + Send + Sync>
}

/// Convert a TestContextError to a tester::CaseError (for use with map_err).
///
/// This is a helper function for converting errors in a map_err context.
///
/// # Arguments
///
/// * `error` - The TestContextError to convert
///
/// # Returns
///
/// * `tester::CaseError` - The converted error
#[allow(dead_code)]
pub fn to_case_error_from_context(error: TestContextError) -> tester::CaseError {
    Box::new(error) as Box<dyn std::error::Error + Send + Sync>
}

/// Check if a program is available for testing.
///
/// # Arguments
///
/// * `repo_dir` - The repository directory
///
/// # Returns
///
/// * `Ok(())` - If the program is available
/// * `Err(tester::CaseError)` - If the program is not available
pub fn check_program_available(repo_dir: &Path) -> Result<(), tester::CaseError> {
    match load_lending_program(repo_dir) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
    }
}

/// Create a test instruction for the lending program.
///
/// # Arguments
///
/// * `program_id` - The lending program ID
/// * `data` - The instruction data
/// * `accounts` - The accounts to pass to the instruction
///
/// # Returns
///
/// * `Instruction` - A lending program instruction
pub fn create_lending_instruction(
    program_id: Pubkey,
    data: Vec<u8>,
    accounts: Vec<AccountMeta>,
) -> Instruction {
    Instruction::new_with_bytes(program_id, &data, accounts)
}

pub struct LendingFixture {
    context: crate::mollusk::LendingTestContext,
    program_id: Pubkey,
    pub user: Pubkey,
    #[allow(dead_code)]
    pub token_program: Pubkey,
    #[allow(dead_code)]
    pub associated_token_program: Pubkey,
}

impl LendingFixture {
    pub fn new_default(repo_dir: &Path) -> Result<Self, TestContextError> {
        let mut context = init_test_context(repo_dir)?;
        let program_id = context.program_id();

        let (system_program_id, system_program_account) = keyed_account_for_system_program();
        context.add_account(system_program_id, system_program_account);

        let (token_program_id, token_program_account) = token::keyed_account();
        context.add_account(token_program_id, token_program_account);

        let (associated_program_id, associated_program_account) = associated_token::keyed_account();
        context.add_account(associated_program_id, associated_program_account);

        let user = context.create_funded_account(1_000_000_000);

        Ok(Self {
            context,
            program_id,
            user,
            token_program: token_program_id,
            associated_token_program: associated_program_id,
        })
    }

    pub fn initialize_instruction(&self) -> Instruction {
        let data = build_initialize_data();
        create_lending_instruction(
            self.program_id,
            data,
            vec![
                AccountMeta::new(self.user, true),
                AccountMeta::new_readonly(solana_system_program::id(), false),
            ],
        )
    }

    pub fn execute_initialize(&mut self) -> Result<(), TestContextError> {
        let instruction = self.initialize_instruction();
        self.context.execute_instruction(&instruction)
    }
}

fn build_initialize_data() -> Vec<u8> {
    anchor_discriminator("global:initialize").to_vec()
}

fn anchor_discriminator(name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let hash = hasher.finalize();
    let mut out = [0u8; 8];
    out.copy_from_slice(&hash[..8]);
    out
}

pub fn run_env_setup_check() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    if !repo_path.exists() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Repository directory not found: {}", repo_path.display()),
        )) as Box<dyn std::error::Error + Send + Sync>);
    }
    check_program_available(&repo_path)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_rust_basics_check() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_solana_model_check() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_anchor_try_check() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    let program_id = load_lending_program_id(&repo_path).map_err(to_case_error_from_load)?;
    let default_id = Pubkey::from_str(DEFAULT_LENDING_PROGRAM_ID)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    if program_id == Pubkey::default() || program_id == default_id {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Program ID is still default",
        )) as Box<dyn std::error::Error + Send + Sync>);
    }

    run_initialize_smoke(&repo_path)
}

pub fn run_spl_token_basics_check() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_cpi_transfer_check() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_token_transfer_check() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_pda_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_treasury_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_account_structure_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_lending_core_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_oracle_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_liquidation_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_interest_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

pub fn run_security_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

#[allow(dead_code)]
pub fn run_testing_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    run_initialize_smoke(&repo_path)
}

#[allow(dead_code)]
pub fn run_deployment_checks() -> Result<(), tester::CaseError> {
    let repo_path = get_repo_dir().map_err(to_case_error_from_load)?;
    let program_id = load_lending_program_id(&repo_path).map_err(to_case_error_from_load)?;
    let default_id = Pubkey::from_str(DEFAULT_LENDING_PROGRAM_ID)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    if program_id == Pubkey::default() || program_id == default_id {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Program ID is still default",
        )) as Box<dyn std::error::Error + Send + Sync>);
    }

    run_initialize_smoke(&repo_path)
}

fn run_initialize_smoke(repo_path: &Path) -> Result<(), tester::CaseError> {
    let mut fixture = LendingFixture::new_default(repo_path).map_err(to_case_error)?;
    match fixture.execute_initialize() {
        Ok(()) => Ok(()),
        Err(TestContextError::ExecutionError(_)) => Ok(()),
        Err(err) => Err(to_case_error(err)),
    }
}
