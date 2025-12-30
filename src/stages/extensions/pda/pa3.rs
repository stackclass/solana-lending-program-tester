use crate::verifier::run_verification;

pub fn test_pda_bump_seeds(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "pa3")
}
