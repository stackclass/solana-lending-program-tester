use crate::verifier::run_verification;

pub fn test_treasury_creation(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "tr2")
}
