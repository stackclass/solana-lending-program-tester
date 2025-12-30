use crate::verifier::run_verification;

pub fn test_reentrancy_protection(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "se2")
}
