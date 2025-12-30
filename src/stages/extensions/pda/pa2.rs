use crate::verifier::run_verification;

pub fn test_pda_derivation(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "pa2")
}
