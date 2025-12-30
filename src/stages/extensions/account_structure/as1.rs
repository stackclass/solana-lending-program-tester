use crate::verifier::run_verification;

pub fn test_bank_account(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "as1")
}
