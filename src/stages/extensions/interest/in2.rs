use crate::verifier::run_verification;

pub fn test_accrued_interest(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "in2")
}
