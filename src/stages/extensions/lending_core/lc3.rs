use crate::verifier::run_verification;

pub fn test_ltv_calculation(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "lc3")
}
