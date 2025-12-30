use crate::verifier::run_verification;

pub fn test_account_practice(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "as4")
}
