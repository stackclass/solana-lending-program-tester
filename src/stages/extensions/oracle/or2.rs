use crate::verifier::run_verification;

pub fn test_pyth_integration(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "or2")
}
