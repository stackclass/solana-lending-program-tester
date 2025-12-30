use crate::verifier::run_verification;

pub fn test_oracle_practice(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "or4")
}
