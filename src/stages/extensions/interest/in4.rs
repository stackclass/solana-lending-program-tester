use crate::verifier::run_verification;

pub fn test_interest_practice(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "in4")
}
