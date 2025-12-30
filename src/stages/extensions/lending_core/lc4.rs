use crate::verifier::run_verification;

pub fn test_core_practice(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "lc4")
}
