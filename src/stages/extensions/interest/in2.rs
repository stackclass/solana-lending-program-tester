use crate::verifier::get_program_info;

pub fn test_accrued_interest(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_accrue = info
        .structs
        .iter()
        .any(|s| s.fields.iter().any(|f| f.name.to_lowercase().contains("accrue")));
    if has_accrue {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Accrued interest not found".to_string())))
    }
}
