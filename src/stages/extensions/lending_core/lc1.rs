use crate::verifier::run_verification;

pub fn test_borrow_basics(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "lc1")
}
