use crate::verifier::get_program_info;

pub fn test_treasury_intro(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_treasury = info.accounts.iter().any(|acc| acc.name.to_lowercase().contains("treasury")) ||
        info.structs.iter().any(|s| s.name.to_lowercase().contains("treasury"));
    if has_treasury {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Treasury code not found".to_string())))
    }
}
