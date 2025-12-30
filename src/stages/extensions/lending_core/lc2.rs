use crate::verifier::run_verification;

pub fn test_repay_basics(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "lc2")
}
