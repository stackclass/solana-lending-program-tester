use crate::verifier::run_verification;

pub fn test_account_validation(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "se3")
}
