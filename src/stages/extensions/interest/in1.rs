use crate::verifier::get_program_info;

pub fn test_interest_basics(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_interest = info
        .structs
        .iter()
        .any(|s| s.fields.iter().any(|f| f.name.to_lowercase().contains("interest"))) ||
        info.accounts
            .iter()
            .any(|acc| acc.fields.iter().any(|f| f.name.to_lowercase().contains("interest")));
    if has_interest {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Interest code not found".to_string())))
    }
}
