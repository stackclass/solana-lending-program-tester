use crate::verifier::get_program_info;

pub fn test_pda_concept(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_pda = info.accounts.iter().any(|acc| {
        acc.name.to_lowercase().contains("pda") || acc.name.to_lowercase().contains("bump")
    }) || info.structs.iter().any(|s| s.name.to_lowercase().contains("pda"));
    if has_pda {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("PDA code not found".to_string())))
    }
}
