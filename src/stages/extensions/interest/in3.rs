use crate::verifier::run_verification;

pub fn test_rate_models(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "in3")
}
