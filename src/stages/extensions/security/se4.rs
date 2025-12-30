use crate::verifier::run_verification;

pub fn test_security_practice(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "se4")
}
