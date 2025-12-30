use crate::verifier::run_verification;

pub fn test_interest_basics(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "in1")
}
