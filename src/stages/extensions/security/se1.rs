use crate::verifier::run_verification;

pub fn test_common_vulnerabilities(harness: &tester::Harness) -> Result<(), tester::CaseError> {
    run_verification(harness, "se1")
}
