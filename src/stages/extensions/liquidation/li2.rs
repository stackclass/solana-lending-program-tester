use crate::verifier::run_verification;

pub fn test_liquidation_trigger(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "li2")
}
