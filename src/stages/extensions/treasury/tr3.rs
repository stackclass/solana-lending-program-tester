use crate::verifier::get_program_info;

pub fn test_treasury_security(_harness: &tester::Harness) -> Result<(), tester::CaseError> {
    let info = get_program_info()?;

    let has_owner = info
        .accounts
        .iter()
        .any(|acc| acc.fields.iter().any(|f| f.name.to_lowercase().contains("owner")));
    if has_owner {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other("Treasury security not found".to_string())))
    }
}
