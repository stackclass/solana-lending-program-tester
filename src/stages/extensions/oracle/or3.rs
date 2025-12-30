use crate::verifier::run_verification;

pub fn test_price_fetching(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "or3")
}
