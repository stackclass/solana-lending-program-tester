use crate::verifier::run_verification;

pub fn test_treasury_security(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "tr3")
}

pub fn test_treasury_practice(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "tr4")
}
